use super::Values;
use crate::de::{error, Error, Result};

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

#[derive(Debug, PartialEq)]
pub(in crate::de) struct Tag<'a> {
    // Should contain all bytes except the leading `#`.
    bytes: &'a [u8],

    comment_state: CommentState,
    escaping_state: EscapingState,
    first_values: bool,

    started_byte_index: usize,
    // The position includes the implicit leading `#`. In reality, `bytes` starts at `column + 1`.
    started_line: usize,
    started_column: usize,

    current_byte_index: usize,
    current_line: usize,
    current_column: usize,

    origin_line: usize,
    origin_column: usize,
}

impl<'a> Tag<'a> {
    pub(super) fn new(bytes: &'a [u8], line: usize, column: usize) -> Self {
        Self {
            bytes,

            comment_state: CommentState::None,
            escaping_state: EscapingState::None,
            first_values: true,

            started_byte_index: 0,
            started_line: line,
            started_column: column + 1,

            current_byte_index: 0,
            current_line: line,
            current_column: column + 1,

            origin_line: line,
            origin_column: column,
        }
    }

    pub(in crate::de) fn next(&mut self) -> Result<Values<'a>> {
        let mut values = None;
        self.started_byte_index = self.current_byte_index;
        self.started_line = self.current_line;
        self.started_column = self.current_column;
        let mut encountered_non_whitespace = false;
        let mut last_byte_newline = false;
        loop {
            if let Some(byte) = self.bytes.get(self.current_byte_index) {
                // Process byte.
                // Check if in comment.
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
                            b';' => {
                                // This is the end of a `Values`.
                                values = Some(Values::new(
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
                if !byte.is_ascii_whitespace() {
                    encountered_non_whitespace = true;
                }
                last_byte_newline = matches!(byte, b'\n');
                if last_byte_newline {
                    self.current_line += 1;
                    self.current_column = 0;
                } else {
                    self.current_column += 1;
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
                                .get_unchecked(self.started_byte_index..ending_byte_index)
                        },
                        self.started_line,
                        self.started_column,
                    ));
                }
                return Err(Error::new(
                    error::Kind::EndOfTag,
                    self.current_line,
                    self.current_column,
                ));
            }
        }
    }

    pub(in crate::de) fn assert_exhausted(&self) -> Result<()> {
        let mut current_line = self.current_line;
        let mut current_column = self.current_column;
        // SAFETY: self.current_byte_index is guaranteed to be within the bounds of self.bytes.
        for byte in unsafe { self.bytes.get_unchecked(self.current_byte_index..) } {
            if !byte.is_ascii_whitespace() {
                return Err(Error::new(
                    error::Kind::UnexpectedValues,
                    current_line,
                    current_column,
                ));
            } else if matches!(byte, b'\n') {
                current_line += 1;
                current_column = 0;
            } else {
                current_column += 1;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Tag;
    use crate::de::{error, parse::Values, Error};
    use claim::{assert_err_eq, assert_ok, assert_ok_eq};

    #[test]
    fn empty() {
        let mut tag = Tag::new(b"", 0, 0);

        assert_ok_eq!(tag.next(), Values::new(b"", 0, 1));
        assert_err_eq!(tag.next(), Error::new(error::Kind::EndOfTag, 0, 1));
    }

    #[test]
    fn finds_single_values() {
        let mut tag = Tag::new(b"foo;\n", 0, 0);

        assert_ok_eq!(tag.next(), Values::new(b"foo", 0, 1));
        assert_err_eq!(tag.next(), Error::new(error::Kind::EndOfTag, 1, 0));
    }

    #[test]
    fn finds_single_values_with_splitting_colons() {
        let mut tag = Tag::new(b"foo:bar:baz;\n", 0, 0);

        assert_ok_eq!(tag.next(), Values::new(b"foo:bar:baz", 0, 1));
        assert_err_eq!(tag.next(), Error::new(error::Kind::EndOfTag, 1, 0));
    }

    #[test]
    fn finds_multiple_values() {
        let mut tag = Tag::new(b"foo;bar;", 0, 0);

        assert_ok_eq!(tag.next(), Values::new(b"foo", 0, 1));
        assert_ok_eq!(tag.next(), Values::new(b"bar", 0, 5));
        assert_err_eq!(tag.next(), Error::new(error::Kind::EndOfTag, 0, 9));
    }

    #[test]
    fn escaped_colon() {
        let mut tag = Tag::new(b"foo\\;bar;", 0, 0);

        assert_ok_eq!(tag.next(), Values::new(b"foo\\;bar", 0, 1));
        assert_err_eq!(tag.next(), Error::new(error::Kind::EndOfTag, 0, 10));
    }

    #[test]
    fn comment() {
        let mut tag = Tag::new(b"foo: //comment;\nbar;\n", 0, 0);

        assert_ok_eq!(tag.next(), Values::new(b"foo: //comment;\nbar", 0, 1));
        assert_err_eq!(tag.next(), Error::new(error::Kind::EndOfTag, 2, 0));
    }

    #[test]
    fn escaped_comment() {
        let mut tag = Tag::new(b"foo: \\/\\/comment;\nbar;\n", 0, 0);

        assert_ok_eq!(tag.next(), Values::new(b"foo: \\/\\/comment", 0, 1));
        assert_ok_eq!(tag.next(), Values::new(b"\nbar", 0, 18));
        assert_err_eq!(tag.next(), Error::new(error::Kind::EndOfTag, 2, 0));
    }

    #[test]
    fn no_closing_semicolon() {
        let mut tag = Tag::new(b"foo\n", 0, 0);

        assert_ok_eq!(tag.next(), Values::new(b"foo", 0, 1));
        assert_err_eq!(tag.next(), Error::new(error::Kind::EndOfTag, 1, 0));
    }

    #[test]
    fn exhausted() {
        let mut tag = Tag::new(b"foo;\n", 0, 0);

        assert_ok!(tag.next());

        assert_ok!(tag.assert_exhausted());
    }

    #[test]
    fn not_exhausted() {
        let mut tag = Tag::new(b"foo;\nbar;\n", 0, 0);

        assert_ok!(tag.next());

        assert_err_eq!(
            tag.assert_exhausted(),
            Error::new(error::Kind::UnexpectedValues, 1, 0)
        );
    }
}
