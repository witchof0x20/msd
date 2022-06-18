use super::Value;
use crate::de::{error, Error, Position, Result};
use std::slice;

enum State {
    None,
    MaybeEnteringComment,
    InComment,
    Escaping,
}

#[derive(Debug)]
pub(in crate::de) struct StoredValues {
    byte_ptr: *const u8,
    byte_len: usize,

    exhausted: bool,

    current_byte_index: usize,
    current_position: Position,
}

impl StoredValues {
    // # Safety
    // The caller must guarantee that the buffer referenced by the returned `Values` does not
    // outlive the returned `Values`. In other words, this returned `Values` must outlive the
    // `Values` from which it was originally created.
    pub(in crate::de) unsafe fn into_values<'a>(self) -> Values<'a> {
        Values {
            // SAFETY: The lifetime of this slice is guaranteed by the caller.
            bytes: unsafe { slice::from_raw_parts(self.byte_ptr, self.byte_len) },

            exhausted: self.exhausted,

            current_byte_index: self.current_byte_index,
            current_position: self.current_position,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(in crate::de) struct Values<'a> {
    bytes: &'a [u8],

    exhausted: bool,

    current_byte_index: usize,
    current_position: Position,
}

impl<'a> Values<'a> {
    pub(in crate::de) fn new(bytes: &'a [u8], position: Position) -> Self {
        Self {
            bytes,

            exhausted: false,

            current_byte_index: 0,
            current_position: position,
        }
    }

    pub(in crate::de) fn next(&mut self) -> Result<Value<'a>> {
        let mut value = None;
        let started_byte_index = self.current_byte_index;
        let started_position = self.current_position;
        let mut state = State::None;
        loop {
            if let Some(byte) = self.bytes.get(self.current_byte_index) {
                // TODO: Put the parsing logic in here instead.
                match state {
                    State::None => {
                        match byte {
                            b':' => {
                                // This is the end of a `Value`.
                                value = Some(Value::new(
                                    // SAFETY: Both ends of the range used here have already been
                                    // determined to be within the bounds of self.bytes.
                                    unsafe {
                                        self.bytes.get_unchecked(
                                            started_byte_index..self.current_byte_index,
                                        )
                                    },
                                    started_position,
                                ));
                            }
                            b'\\' => {
                                // Enter an escaping state.
                                state = State::Escaping;
                            }
                            b'/' => {
                                state = State::MaybeEnteringComment;
                            }
                            _ => {}
                        }
                    }
                    State::MaybeEnteringComment => {
                        match byte {
                            b':' => {
                                // This is the end of a `Value`.
                                value = Some(Value::new(
                                    // SAFETY: Both ends of the range used here have already been
                                    // determined to be within the bounds of self.bytes.
                                    unsafe {
                                        self.bytes.get_unchecked(
                                            started_byte_index..self.current_byte_index,
                                        )
                                    },
                                    started_position,
                                ));
                            }
                            b'\\' => {
                                // Enter an escaping state.
                                state = State::Escaping;
                            }
                            b'/' => {
                                // Handle comment state.
                                state = State::InComment;
                            }
                            _ => {
                                state = State::None;
                            }
                        }
                    }
                    State::InComment => {
                        // Consume bytes until we are on a new line.
                        if matches!(byte, b'\n') {
                            state = State::None;
                        }
                    }
                    State::Escaping => {
                        state = State::None;
                    }
                }

                if matches!(byte, b'\n') {
                    self.current_position = self.current_position.increment_line();
                } else {
                    self.current_position = self.current_position.increment_column();
                }
                self.current_byte_index += 1;

                if let Some(value) = value {
                    return Ok(value);
                }
            } else if !self.exhausted {
                self.exhausted = true;
                return Ok(Value::new(
                    // SAFETY: self.current_byte_index is guaranteed to only be one past the
                    // last value in the slice.
                    unsafe {
                        self.bytes
                            .get_unchecked(started_byte_index..self.current_byte_index)
                    },
                    started_position,
                ));
            } else {
                return Err(Error::new(error::Kind::EndOfValues, self.current_position));
            }
        }
    }

    pub(in crate::de) fn assert_exhausted(&self) -> Result<()> {
        if self.exhausted {
            Ok(())
        } else {
            Err(Error::new(
                error::Kind::UnexpectedValue,
                self.current_position,
            ))
        }
    }

    pub(in crate::de) fn into_stored(self) -> StoredValues {
        StoredValues {
            byte_ptr: self.bytes.as_ptr(),
            byte_len: self.bytes.len(),

            exhausted: self.exhausted,

            current_byte_index: self.current_byte_index,
            current_position: self.current_position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Values;
    use crate::de::{error, parse::Value, Error, Position};
    use claim::{assert_err_eq, assert_ok, assert_ok_eq};

    #[test]
    fn empty() {
        let mut values = Values::new(b"", Position::new(0, 0));

        assert_ok_eq!(values.next(), Value::new(b"", Position::new(0, 0)));
        assert_err_eq!(
            values.next(),
            Error::new(error::Kind::EndOfValues, Position::new(0, 0))
        );
    }

    #[test]
    fn finds_single_value() {
        let mut values = Values::new(b"foo", Position::new(0, 0));

        assert_ok_eq!(values.next(), Value::new(b"foo", Position::new(0, 0)));
        assert_err_eq!(
            values.next(),
            Error::new(error::Kind::EndOfValues, Position::new(0, 3))
        );
    }

    #[test]
    fn finds_multiple_values() {
        let mut values = Values::new(b"foo:bar:baz", Position::new(0, 0));

        assert_ok_eq!(values.next(), Value::new(b"foo", Position::new(0, 0)));
        assert_ok_eq!(values.next(), Value::new(b"bar", Position::new(0, 4)));
        assert_ok_eq!(values.next(), Value::new(b"baz", Position::new(0, 8)));
        assert_err_eq!(
            values.next(),
            Error::new(error::Kind::EndOfValues, Position::new(0, 11))
        );
    }

    #[test]
    fn comment() {
        let mut values = Values::new(b"foo//comment:\n:bar", Position::new(0, 0));

        assert_ok_eq!(
            values.next(),
            Value::new(b"foo//comment:\n", Position::new(0, 0))
        );
        assert_ok_eq!(values.next(), Value::new(b"bar", Position::new(1, 1)));
        assert_err_eq!(
            values.next(),
            Error::new(error::Kind::EndOfValues, Position::new(1, 4))
        );
    }

    #[test]
    fn end_of_values_after_maybe_entering_comment() {
        let mut values = Values::new(b"foo/:bar", Position::new(0, 0));

        assert_ok_eq!(values.next(), Value::new(b"foo/", Position::new(0, 0)));
        assert_ok_eq!(values.next(), Value::new(b"bar", Position::new(0, 5)));
        assert_err_eq!(
            values.next(),
            Error::new(error::Kind::EndOfValues, Position::new(0, 8))
        );
    }

    #[test]
    fn escaping_after_maybe_entering_comment() {
        let mut values = Values::new(b"foo/\\::bar", Position::new(0, 0));

        assert_ok_eq!(values.next(), Value::new(b"foo/\\:", Position::new(0, 0)));
        assert_ok_eq!(values.next(), Value::new(b"bar", Position::new(0, 7)));
        assert_err_eq!(
            values.next(),
            Error::new(error::Kind::EndOfValues, Position::new(0, 10))
        );
    }

    #[test]
    fn normal_byte_after_maybe_entering_comment() {
        let mut values = Values::new(b"fo/o:bar", Position::new(0, 0));

        assert_ok_eq!(values.next(), Value::new(b"fo/o", Position::new(0, 0)));
        assert_ok_eq!(values.next(), Value::new(b"bar", Position::new(0, 5)));
        assert_err_eq!(
            values.next(),
            Error::new(error::Kind::EndOfValues, Position::new(0, 8))
        );
    }

    #[test]
    fn escaped_comment() {
        let mut values = Values::new(b"foo\\/\\/comment:\n:bar", Position::new(0, 0));

        assert_ok_eq!(
            values.next(),
            Value::new(b"foo\\/\\/comment", Position::new(0, 0))
        );
        assert_ok_eq!(values.next(), Value::new(b"\n", Position::new(0, 15)));
        assert_ok_eq!(values.next(), Value::new(b"bar", Position::new(1, 1)));
        assert_err_eq!(
            values.next(),
            Error::new(error::Kind::EndOfValues, Position::new(1, 4))
        );
    }

    #[test]
    fn escaped_colon() {
        let mut values = Values::new(b"foo\\:bar", Position::new(0, 0));

        assert_ok_eq!(values.next(), Value::new(b"foo\\:bar", Position::new(0, 0)));
        assert_err_eq!(
            values.next(),
            Error::new(error::Kind::EndOfValues, Position::new(0, 8))
        );
    }

    #[test]
    fn escaped_backslash() {
        let mut values = Values::new(b"foo\\\\:bar", Position::new(0, 0));

        assert_ok_eq!(values.next(), Value::new(b"foo\\\\", Position::new(0, 0)));
        assert_ok_eq!(values.next(), Value::new(b"bar", Position::new(0, 6)));
        assert_err_eq!(
            values.next(),
            Error::new(error::Kind::EndOfValues, Position::new(0, 9))
        );
    }

    #[test]
    fn exhausted() {
        let mut values = Values::new(b"foo", Position::new(0, 0));

        assert_ok!(values.next());

        assert_ok!(values.assert_exhausted());
    }

    #[test]
    fn not_exhausted() {
        let mut values = Values::new(b"foo:bar", Position::new(0, 0));

        assert_ok!(values.next());

        assert_err_eq!(
            values.assert_exhausted(),
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 4))
        );
    }

    #[test]
    fn into_stored() {
        let buffer = b"foo";
        let values = Values::new(buffer, Position::new(0, 0));
        let stored = values.into_stored();
        let mut unstored_values = unsafe { stored.into_values() };

        assert_ok_eq!(
            unstored_values.next(),
            Value::new(b"foo", Position::new(0, 0))
        );
        assert_err_eq!(
            unstored_values.next(),
            Error::new(error::Kind::EndOfValues, Position::new(0, 3))
        );
    }

    #[test]
    fn into_stored_after_iteration() {
        let buffer = b"foo:bar";
        let mut values = Values::new(buffer, Position::new(0, 0));
        assert_ok!(values.next());
        let stored = values.into_stored();
        let mut unstored_values = unsafe { stored.into_values() };

        assert_ok_eq!(
            unstored_values.next(),
            Value::new(b"bar", Position::new(0, 4))
        );
        assert_err_eq!(
            unstored_values.next(),
            Error::new(error::Kind::EndOfValues, Position::new(0, 7))
        );
    }
}
