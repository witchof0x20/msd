use super::Value;
use crate::de::{error, Error, Result};
use std::slice;

#[derive(Debug, PartialEq)]
enum CommentState {
    MaybeEnteringComment,
    InComment,
    None,
}

#[derive(Debug, PartialEq)]
enum EscapingState {
    Escaping,
    None,
}

pub(in crate::de) struct StoredValues {
    byte_ptr: *const u8,
    byte_len: usize,

    exhausted: bool,

    current_byte_index: usize,
    current_line: usize,
    current_column: usize,
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

            comment_state: CommentState::None,
            escaping_state: EscapingState::None,
            exhausted: self.exhausted,

            started_byte_index: self.current_byte_index,
            started_line: self.current_line,
            started_column: self.current_column,

            current_byte_index: self.current_byte_index,
            current_line: self.current_line,
            current_column: self.current_column,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(in crate::de) struct Values<'a> {
    bytes: &'a [u8],

    comment_state: CommentState,
    escaping_state: EscapingState,
    exhausted: bool,

    started_byte_index: usize,
    started_line: usize,
    started_column: usize,

    current_byte_index: usize,
    current_line: usize,
    current_column: usize,
}

impl<'a> Values<'a> {
    pub(in crate::de) fn new(bytes: &'a [u8], line: usize, column: usize) -> Self {
        Self {
            bytes,

            comment_state: CommentState::None,
            escaping_state: EscapingState::None,
            exhausted: false,

            started_byte_index: 0,
            started_line: line,
            started_column: column,

            current_byte_index: 0,
            current_line: line,
            current_column: column,
        }
    }

    pub(in crate::de) fn next(&mut self) -> Result<Value<'a>> {
        let mut value = None;
        self.started_byte_index = self.current_byte_index;
        self.started_line = self.current_line;
        self.started_column = self.current_column;
        loop {
            if let Some(byte) = self.bytes.get(self.current_byte_index) {
                if matches!(self.comment_state, CommentState::InComment) {
                    // Consume bytes until we are on a new line.
                    if matches!(byte, b'\n') {
                        self.comment_state = CommentState::None;
                    }
                } else {
                    // Check if current character is escaped.
                    if matches!(self.escaping_state, EscapingState::Escaping) {
                        self.escaping_state = EscapingState::None;
                    } else {
                        match byte {
                            b':' => {
                                // This is the end of a `Value`.
                                value = Some(Value::new(
                                    // SAFETY: Both ends of the range used here have already been
                                    // determined to be within the bounds of self.bytes.
                                    unsafe {
                                        self.bytes.get_unchecked(
                                            self.started_byte_index..self.current_byte_index,
                                        )
                                    },
                                    self.started_line,
                                    self.started_column,
                                ));
                            }
                            b'\\' => {
                                // Enter an escaping state.
                                self.escaping_state = EscapingState::Escaping;
                            }
                            b'/' => {
                                // Handle comment state.
                                if matches!(self.comment_state, CommentState::MaybeEnteringComment)
                                {
                                    self.comment_state = CommentState::InComment;
                                } else {
                                    self.comment_state = CommentState::MaybeEnteringComment;
                                }
                            }
                            _ => {}
                        }
                    }
                }

                if matches!(byte, b'\n') {
                    self.current_line += 1;
                    self.current_column = 0;
                } else {
                    self.current_column += 1;
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
                            .get_unchecked(self.started_byte_index..self.current_byte_index)
                    },
                    self.started_line,
                    self.started_column,
                ));
            } else {
                return Err(Error::new(
                    error::Kind::EndOfValues,
                    self.current_line,
                    self.current_column,
                ));
            }
        }
    }

    pub(in crate::de) fn assert_exhausted(&self) -> Result<()> {
        if self.exhausted {
            Ok(())
        } else {
            Err(Error::new(
                error::Kind::UnexpectedValue,
                self.current_line,
                self.current_column,
            ))
        }
    }

    pub(in crate::de) fn into_stored(self) -> StoredValues {
        StoredValues {
            byte_ptr: self.bytes.as_ptr(),
            byte_len: self.bytes.len(),

            exhausted: self.exhausted,

            current_byte_index: self.current_byte_index,
            current_line: self.current_line,
            current_column: self.current_column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Values;
    use crate::de::{error, parse::Value, Error};
    use claim::{assert_err_eq, assert_ok, assert_ok_eq};

    #[test]
    fn empty() {
        let mut values = Values::new(b"", 0, 0);

        assert_ok_eq!(values.next(), Value::new(b"", 0, 0));
        assert_err_eq!(values.next(), Error::new(error::Kind::EndOfValues, 0, 0));
    }

    #[test]
    fn finds_single_value() {
        let mut values = Values::new(b"foo", 0, 0);

        assert_ok_eq!(values.next(), Value::new(b"foo", 0, 0));
        assert_err_eq!(values.next(), Error::new(error::Kind::EndOfValues, 0, 3));
    }

    #[test]
    fn finds_multiple_values() {
        let mut values = Values::new(b"foo:bar:baz", 0, 0);

        assert_ok_eq!(values.next(), Value::new(b"foo", 0, 0));
        assert_ok_eq!(values.next(), Value::new(b"bar", 0, 4));
        assert_ok_eq!(values.next(), Value::new(b"baz", 0, 8));
        assert_err_eq!(values.next(), Error::new(error::Kind::EndOfValues, 0, 11));
    }

    #[test]
    fn comment() {
        let mut values = Values::new(b"foo//comment:\n:bar", 0, 0);

        assert_ok_eq!(values.next(), Value::new(b"foo//comment:\n", 0, 0));
        assert_ok_eq!(values.next(), Value::new(b"bar", 1, 1));
        assert_err_eq!(values.next(), Error::new(error::Kind::EndOfValues, 1, 4));
    }

    #[test]
    fn escaped_comment() {
        let mut values = Values::new(b"foo\\/\\/comment:\n:bar", 0, 0);

        assert_ok_eq!(values.next(), Value::new(b"foo\\/\\/comment", 0, 0));
        assert_ok_eq!(values.next(), Value::new(b"\n", 0, 15));
        assert_ok_eq!(values.next(), Value::new(b"bar", 1, 1));
        assert_err_eq!(values.next(), Error::new(error::Kind::EndOfValues, 1, 4));
    }

    #[test]
    fn escaped_colon() {
        let mut values = Values::new(b"foo\\:bar", 0, 0);

        assert_ok_eq!(values.next(), Value::new(b"foo\\:bar", 0, 0));
        assert_err_eq!(values.next(), Error::new(error::Kind::EndOfValues, 0, 8));
    }

    #[test]
    fn escaped_backslash() {
        let mut values = Values::new(b"foo\\\\:bar", 0, 0);

        assert_ok_eq!(values.next(), Value::new(b"foo\\\\", 0, 0));
        assert_ok_eq!(values.next(), Value::new(b"bar", 0, 6));
        assert_err_eq!(values.next(), Error::new(error::Kind::EndOfValues, 0, 9));
    }

    #[test]
    fn exhausted() {
        let mut values = Values::new(b"foo", 0, 0);

        assert_ok!(values.next());

        assert_ok!(values.assert_exhausted());
    }

    #[test]
    fn not_exhausted() {
        let mut values = Values::new(b"foo:bar", 0, 0);

        assert_ok!(values.next());

        assert_err_eq!(
            values.assert_exhausted(),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    fn into_stored() {
        let buffer = b"foo";
        let values = Values::new(buffer, 0, 0);
        let stored = values.into_stored();
        let mut unstored_values = unsafe { stored.into_values() };

        assert_ok_eq!(unstored_values.next(), Value::new(b"foo", 0, 0));
        assert_err_eq!(
            unstored_values.next(),
            Error::new(error::Kind::EndOfValues, 0, 3)
        );
    }

    #[test]
    fn into_stored_after_iteration() {
        let buffer = b"foo:bar";
        let mut values = Values::new(buffer, 0, 0);
        assert_ok!(values.next());
        let stored = values.into_stored();
        let mut unstored_values = unsafe { stored.into_values() };

        assert_ok_eq!(unstored_values.next(), Value::new(b"bar", 0, 4));
        assert_err_eq!(
            unstored_values.next(),
            Error::new(error::Kind::EndOfValues, 0, 7)
        );
    }
}
