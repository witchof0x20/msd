use super::{StoredValues, Values};
use crate::de::{error, Error, Position, Result};
use std::slice;

enum State {
    None,
    MaybeEnteringComment,
    InComment,
    Escaping,
}

// Tag without the lifetime. Used when storing within an Access.
#[derive(Debug)]
pub(in crate::de) struct StoredTag {
    byte_ptr: *const u8,
    byte_len: usize,

    first_values: bool,

    current_byte_index: usize,
    current_position: Position,

    origin_position: Position,

    revisit: Option<StoredValues>,
}

impl StoredTag {
    // # Safety
    // The caller must guarantee that the buffer referenced by the returned `Tag` does not outlive
    // the returned `Tag`. In other words, this returned `Tag` must outlive the `Tags` from which
    // it was originally created.
    pub(in crate::de) unsafe fn into_tag<'a>(self) -> Tag<'a> {
        Tag {
            // SAFETY: The lifetime of this slice is guaranteed by the caller.
            bytes: unsafe { slice::from_raw_parts(self.byte_ptr, self.byte_len) },

            first_values: self.first_values,

            current_byte_index: self.current_byte_index,
            current_position: self.current_position,

            origin_position: self.origin_position,

            // SAFETY: The revisit is guaranteed to have the same lifetime as the containing `Tag`.
            revisit: unsafe { self.revisit.map(|stored| stored.into_values()) },
        }
    }

    pub(in crate::de) fn origin_position(&self) -> Position {
        self.origin_position
    }
}

#[derive(Debug, PartialEq)]
pub(in crate::de) struct Tag<'a> {
    // Should contain all bytes except the leading `#`.
    bytes: &'a [u8],

    first_values: bool,

    current_byte_index: usize,
    current_position: Position,

    origin_position: Position,

    revisit: Option<Values<'a>>,
}

impl<'a> Tag<'a> {
    pub(in crate::de) fn new(bytes: &'a [u8], position: Position) -> Self {
        Self {
            bytes,

            first_values: true,

            current_byte_index: 0,
            current_position: position.increment_column(),

            origin_position: position,

            revisit: None,
        }
    }

    pub(in crate::de) fn next(&mut self) -> Result<Values<'a>> {
        if let Some(revisit) = self.revisit.take() {
            return Ok(revisit);
        }

        let mut values = None;
        let started_byte_index = self.current_byte_index;
        let started_position = self.current_position;
        let mut state = State::None;
        let mut encountered_non_whitespace = false;
        let mut last_byte_newline = false;
        loop {
            if let Some(byte) = self.bytes.get(self.current_byte_index) {
                // Process byte.
                match state {
                    State::None => {
                        match byte {
                            b';' => {
                                // This is the end of a `Values`.
                                values = Some(Values::new(
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
                            b';' => {
                                // This is the end of a `Values`.
                                values = Some(Values::new(
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
                if !byte.is_ascii_whitespace() {
                    encountered_non_whitespace = true;
                }
                last_byte_newline = matches!(byte, b'\n');
                if last_byte_newline {
                    self.current_position = self.current_position.increment_line();
                } else {
                    self.current_position = self.current_position.increment_column();
                }
                self.current_byte_index += 1;

                if let Some(values) = values {
                    self.first_values = false;
                    return Ok(values);
                }
            } else {
                if self.first_values || encountered_non_whitespace {
                    self.first_values = false;
                    let ending_byte_index = if last_byte_newline {
                        self.current_byte_index - 1
                    } else {
                        self.current_byte_index
                    };
                    return Ok(Values::new(
                        // SAFETY: self.current_byte_index is guaranteed to only be one past the
                        // last value in the slice.
                        unsafe {
                            self.bytes
                                .get_unchecked(started_byte_index..ending_byte_index)
                        },
                        started_position,
                    ));
                }
                return Err(Error::new(error::Kind::EndOfTag, self.current_position));
            }
        }
    }

    pub(in crate::de) fn reset(&mut self) {
        self.first_values = true;
        self.current_byte_index = 0;
        self.current_position = self.origin_position.increment_column();
    }

    // SAFETY: `values` must reference the same buffer referenced by this tag. In other words,
    // `values` should have been created by a call to this tag's `next()` method.
    pub(in crate::de) unsafe fn revisit(&mut self, values: Values<'a>) {
        self.revisit = Some(values);
    }

    pub(in crate::de) fn assert_exhausted(&self) -> Result<()> {
        let mut current_position = self.current_position;
        // SAFETY: self.current_byte_index is guaranteed to be within the bounds of self.bytes.
        for byte in unsafe { self.bytes.get_unchecked(self.current_byte_index..) } {
            if !byte.is_ascii_whitespace() {
                return Err(Error::new(error::Kind::UnexpectedValues, current_position));
            } else if matches!(byte, b'\n') {
                current_position = current_position.increment_line();
            } else {
                current_position = current_position.increment_column();
            }
        }
        Ok(())
    }

    pub(in crate::de) fn into_stored(self) -> StoredTag {
        StoredTag {
            byte_ptr: self.bytes.as_ptr(),
            byte_len: self.bytes.len(),

            first_values: self.first_values,

            current_byte_index: self.current_byte_index,
            current_position: self.current_position,

            origin_position: self.origin_position,

            revisit: self.revisit.map(|values| values.into_stored()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tag;
    use crate::de::{error, parse::Values, Error, Position};
    use claim::{assert_err_eq, assert_ok, assert_ok_eq};

    #[test]
    fn empty() {
        let mut tag = Tag::new(b"", Position::new(0, 0));

        assert_ok_eq!(tag.next(), Values::new(b"", Position::new(0, 1)));
        assert_err_eq!(
            tag.next(),
            Error::new(error::Kind::EndOfTag, Position::new(0, 1))
        );
    }

    #[test]
    fn finds_single_values() {
        let mut tag = Tag::new(b"foo;\n", Position::new(0, 0));

        assert_ok_eq!(tag.next(), Values::new(b"foo", Position::new(0, 1)));
        assert_err_eq!(
            tag.next(),
            Error::new(error::Kind::EndOfTag, Position::new(1, 0))
        );
    }

    #[test]
    fn finds_single_values_with_splitting_colons() {
        let mut tag = Tag::new(b"foo:bar:baz;\n", Position::new(0, 0));

        assert_ok_eq!(tag.next(), Values::new(b"foo:bar:baz", Position::new(0, 1)));
        assert_err_eq!(
            tag.next(),
            Error::new(error::Kind::EndOfTag, Position::new(1, 0))
        );
    }

    #[test]
    fn finds_multiple_values() {
        let mut tag = Tag::new(b"foo;bar;", Position::new(0, 0));

        assert_ok_eq!(tag.next(), Values::new(b"foo", Position::new(0, 1)));
        assert_ok_eq!(tag.next(), Values::new(b"bar", Position::new(0, 5)));
        assert_err_eq!(
            tag.next(),
            Error::new(error::Kind::EndOfTag, Position::new(0, 9))
        );
    }

    #[test]
    fn escaped_colon() {
        let mut tag = Tag::new(b"foo\\;bar;", Position::new(0, 0));

        assert_ok_eq!(tag.next(), Values::new(b"foo\\;bar", Position::new(0, 1)));
        assert_err_eq!(
            tag.next(),
            Error::new(error::Kind::EndOfTag, Position::new(0, 10))
        );
    }

    #[test]
    fn comment() {
        let mut tag = Tag::new(b"foo: //comment;\nbar;\n", Position::new(0, 0));

        assert_ok_eq!(
            tag.next(),
            Values::new(b"foo: //comment;\nbar", Position::new(0, 1))
        );
        assert_err_eq!(
            tag.next(),
            Error::new(error::Kind::EndOfTag, Position::new(2, 0))
        );
    }

    #[test]
    fn escaped_comment() {
        let mut tag = Tag::new(b"foo: \\/\\/comment;\nbar;\n", Position::new(0, 0));

        assert_ok_eq!(
            tag.next(),
            Values::new(b"foo: \\/\\/comment", Position::new(0, 1))
        );
        assert_ok_eq!(tag.next(), Values::new(b"\nbar", Position::new(0, 18)));
        assert_err_eq!(
            tag.next(),
            Error::new(error::Kind::EndOfTag, Position::new(2, 0))
        );
    }

    #[test]
    fn no_closing_semicolon() {
        let mut tag = Tag::new(b"foo\n", Position::new(0, 0));

        assert_ok_eq!(tag.next(), Values::new(b"foo", Position::new(0, 1)));
        assert_err_eq!(
            tag.next(),
            Error::new(error::Kind::EndOfTag, Position::new(1, 0))
        );
    }

    #[test]
    fn reset() {
        let mut tag = Tag::new(b"foo;\n", Position::new(0, 0));

        assert_ok!(tag.next());
        assert_ok!(tag.assert_exhausted());

        tag.reset();

        assert_ok_eq!(tag.next(), Values::new(b"foo", Position::new(0, 1)));
    }

    #[test]
    fn revisit() {
        let mut tag = Tag::new(b"foo;", Position::new(0, 0));

        let values = assert_ok!(tag.next());
        unsafe { tag.revisit(values) };
        assert_ok_eq!(tag.next(), Values::new(b"foo", Position::new(0, 1)));
    }

    #[test]
    fn exhausted() {
        let mut tag = Tag::new(b"foo;\n", Position::new(0, 0));

        assert_ok!(tag.next());

        assert_ok!(tag.assert_exhausted());
    }

    #[test]
    fn not_exhausted() {
        let mut tag = Tag::new(b"foo;\nbar;\n", Position::new(0, 0));

        assert_ok!(tag.next());

        assert_err_eq!(
            tag.assert_exhausted(),
            Error::new(error::Kind::UnexpectedValues, Position::new(1, 0))
        );
    }

    #[test]
    fn into_stored() {
        let buffer = b"foo;";
        let tag = Tag::new(buffer, Position::new(0, 0));
        let stored = tag.into_stored();
        let mut unstored_tag = unsafe { stored.into_tag() };

        assert_ok_eq!(
            unstored_tag.next(),
            Values::new(b"foo", Position::new(0, 1))
        );
        assert_err_eq!(
            unstored_tag.next(),
            Error::new(error::Kind::EndOfTag, Position::new(0, 5))
        );
    }

    #[test]
    fn into_stored_after_iteration() {
        let buffer = b"foo;bar;";
        let mut tag = Tag::new(buffer, Position::new(0, 0));
        assert_ok!(tag.next());
        let stored = tag.into_stored();
        let mut unstored_tag = unsafe { stored.into_tag() };

        assert_ok_eq!(
            unstored_tag.next(),
            Values::new(b"bar", Position::new(0, 5))
        );
        assert_err_eq!(
            unstored_tag.next(),
            Error::new(error::Kind::EndOfTag, Position::new(0, 9))
        );
    }

    #[test]
    fn stored_origin_position() {
        let buffer = b"foo;bar;";
        let mut tag = Tag::new(buffer, Position::new(1, 2));
        assert_ok!(tag.next());

        assert_eq!(tag.into_stored().origin_position(), Position::new(1, 2));
    }
}
