use super::Tag;
use crate::de::{error, parse::StoredTag, Error, Position, Result};
use std::io::{Bytes, Read};

enum State {
    None,
    MaybeEnteringComment,
    InComment,
    Escaping,
}

#[derive(Debug)]
pub(in crate::de) struct Tags<R> {
    reader: Bytes<R>,

    buffer: Vec<u8>,

    first_tag: bool,

    current_position: Position,

    encountered_error: Option<Error>,
    exhausted: bool,

    revisit: Option<StoredTag>,
}

impl<R> Tags<R>
where
    R: Read,
{
    pub(in crate::de) fn new(reader: R) -> Self {
        Self {
            reader: reader.bytes(),

            buffer: Vec::with_capacity(1024),

            first_tag: true,

            current_position: Position::new(0, 0),

            encountered_error: None,
            exhausted: false,

            revisit: None,
        }
    }

    pub(in crate::de) fn current_position(&self) -> Position {
        self.current_position
    }

    /// Returns the next tag in the input, if there is one.
    ///
    /// Note that this is not an `Iterator::next()`, because it is impossible for an iterator to
    /// return items that have a shorter lifetime than the iterator itself. Each `Tag` returned
    /// here only lives until the next call to `next()` because it borrows from a reused internal
    /// buffer.
    pub(in crate::de) fn next(&mut self) -> Result<Tag> {
        if let Some(error) = &self.encountered_error {
            return Err(error.clone());
        }

        if let Some(revisit) = self.revisit.take() {
            return Ok(
                // SAFETY: `revisit` is guaranteed to point to valid contents on the current
                // buffer.
                unsafe { revisit.into_tag() },
            );
        }

        let mut state = State::None;
        let mut end_of_values = false;
        let mut starting_new_line = false;
        let mut started_position = self.current_position;

        // Find the first tag, if necessary.
        if self.first_tag {
            loop {
                let byte = match self.reader.next() {
                    Some(byte) => match byte {
                        Ok(byte) => byte,
                        Err(_error) => {
                            let error = Error::new(error::Kind::Io, self.current_position);
                            self.encountered_error = Some(error.clone());
                            self.exhausted = true;
                            return Err(error);
                        }
                    },
                    None => {
                        self.exhausted = true;
                        let error = Error::new(error::Kind::EndOfFile, self.current_position);
                        self.encountered_error = Some(error.clone());
                        return Err(error);
                    }
                };

                match state {
                    State::None => {
                        match byte {
                            b'#' => {
                                started_position = self.current_position;
                                self.current_position = self.current_position.increment_column();
                                break;
                            }
                            b'/' => {
                                state = State::MaybeEnteringComment;
                            }
                            _ => {
                                // Non-whitespace bytes are not allowed before the first tag.
                                if !byte.is_ascii_whitespace() {
                                    let error =
                                        Error::new(error::Kind::ExpectedTag, self.current_position);
                                    self.encountered_error = Some(error.clone());
                                    return Err(error);
                                }
                            }
                        }
                    }
                    State::MaybeEnteringComment => match byte {
                        b'/' => {
                            state = State::InComment;
                        }
                        _ => {
                            let error = Error::new(
                                error::Kind::ExpectedTag,
                                self.current_position.decrement_column(),
                            );
                            self.encountered_error = Some(error.clone());
                            return Err(error);
                        }
                    },
                    State::InComment => {
                        // Consume bytes until we are on a new line.
                        if matches!(byte, b'\n') {
                            state = State::None;
                        }
                    }
                    State::Escaping => unreachable!(),
                }

                if matches!(byte, b'\n') {
                    self.current_position = self.current_position.increment_line();
                } else {
                    self.current_position = self.current_position.increment_column();
                }
            }

            self.first_tag = false;
        }

        // Reuse the same buffer.
        self.buffer.clear();

        loop {
            let byte = match self.reader.next() {
                Some(byte) => match byte {
                    Ok(byte) => byte,
                    Err(_error) => {
                        let error = Error::new(error::Kind::Io, self.current_position);
                        self.encountered_error = Some(error.clone());
                        self.exhausted = true;
                        return Err(error);
                    }
                },
                None => {
                    self.exhausted = true;
                    if self.buffer.is_empty() {
                        let error = Error::new(error::Kind::EndOfFile, self.current_position);
                        self.encountered_error = Some(error.clone());
                        return Err(error);
                    } else {
                        return Ok(Tag::new(&self.buffer, started_position));
                    }
                }
            };

            // Process byte.
            match state {
                State::None => {
                    match byte {
                        b'#' => {
                            // We are lenient on the formatting here. If a `#` is at the
                            // start of a newline we begin a new tag and assume the
                            // previous tag was missing the closing `;` (some old
                            // implementations of MSD didn't explicitly require the `;`).
                            // If we are in the middle of a line, we assume it was meant to
                            // be escaped.
                            if starting_new_line || end_of_values {
                                // Entering a new tag. Return the previous one.
                                return Ok(Tag::new(&self.buffer, started_position));
                            }
                            end_of_values = false;
                        }
                        b';' => {
                            end_of_values = true;
                        }
                        b'\\' => {
                            state = State::Escaping;
                            end_of_values = false;
                        }
                        b'/' => {
                            state = State::MaybeEnteringComment;
                        }
                        _ => {
                            if !byte.is_ascii_whitespace() {
                                end_of_values = false;
                            }
                        }
                    }
                }
                State::MaybeEnteringComment => match byte {
                    b';' => {
                        end_of_values = true;
                        state = State::None;
                    }
                    b'\\' => {
                        state = State::Escaping;
                        end_of_values = false;
                    }
                    b'/' => {
                        state = State::InComment;
                    }
                    _ => {
                        state = State::None;
                        end_of_values = false;
                    }
                },
                State::InComment => {
                    // Consume bytes until we are on a new line.
                    if matches!(byte, b'\n') {
                        state = State::None;
                    }
                }
                State::Escaping => {
                    state = State::None;
                    end_of_values = false;
                }
            }
            self.buffer.push(byte);

            if matches!(byte, b'\n') {
                self.current_position = self.current_position.increment_line();
                starting_new_line = true;
            } else {
                self.current_position = self.current_position.increment_column();
                starting_new_line = false;
            }
        }
    }

    /// Returns whether there will be another tag.
    ///
    /// If this returns `true`, then a call to `next()` will return `Some(tag)`.
    pub(in crate::de) fn has_next(&mut self) -> Result<bool> {
        // The iterator will only be in the state below if no tags have been returned yet.
        // Simply find the first tag if it exists.
        if self.first_tag {
            let mut state = State::None;

            loop {
                let byte = match self.reader.next() {
                    Some(byte) => match byte {
                        Ok(byte) => byte,
                        Err(_error) => {
                            let error = Error::new(error::Kind::Io, self.current_position);
                            self.encountered_error = Some(error);
                            self.exhausted = true;
                            break;
                        }
                    },
                    None => {
                        self.exhausted = true;
                        break;
                    }
                };

                // Process byte.
                match state {
                    State::None => {
                        match byte {
                            b'#' => {
                                break;
                            }
                            b'/' => {
                                state = State::MaybeEnteringComment;
                            }
                            _ => {
                                // Non-whitespace bytes are not allowed before the first tag.
                                if !byte.is_ascii_whitespace() {
                                    let error =
                                        Error::new(error::Kind::ExpectedTag, self.current_position);
                                    self.encountered_error = Some(error.clone());
                                    return Err(error);
                                }
                            }
                        }
                    }
                    State::MaybeEnteringComment => match byte {
                        b'/' => {
                            state = State::InComment;
                        }
                        _ => {
                            let error = Error::new(
                                error::Kind::ExpectedTag,
                                self.current_position.decrement_column(),
                            );
                            self.encountered_error = Some(error.clone());
                            return Err(error);
                        }
                    },
                    State::InComment => {
                        // Consume bytes until we are on a new line.
                        if matches!(byte, b'\n') {
                            state = State::None;
                        }
                    }
                    State::Escaping => unreachable!(),
                }

                if matches!(byte, b'\n') {
                    self.current_position = self.current_position.increment_line();
                } else {
                    self.current_position = self.current_position.increment_column();
                }
            }

            self.first_tag = false;
        }

        if let Some(error) = &self.encountered_error {
            Err(error.clone())
        } else {
            Ok(!self.exhausted)
        }
    }

    // SAFETY: `tag` must reference this struct's buffer.
    pub(in crate::de) unsafe fn revisit(&mut self, tag: StoredTag) {
        self.revisit = Some(tag)
    }

    pub(in crate::de) fn assert_exhausted(&self) -> Result<()> {
        if let Some(revisit) = self.revisit.as_ref() {
            Err(Error::new(
                error::Kind::UnexpectedTag,
                revisit.origin_position(),
            ))
        } else if self.exhausted {
            Ok(())
        } else {
            Err(Error::new(
                error::Kind::UnexpectedTag,
                self.current_position,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tags;
    use crate::de::{error, parse::Tag, Error, Position};
    use claim::{assert_err_eq, assert_ok, assert_ok_eq};

    #[test]
    fn empty_reader() {
        let input = b"";
        let mut tags = Tags::new(input.as_slice());

        assert_err_eq!(
            tags.next(),
            Error::new(error::Kind::EndOfFile, Position::new(0, 0))
        );
    }

    #[test]
    fn character_before_first_tag() {
        let input = b"foo#bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_err_eq!(
            tags.next(),
            Error::new(error::Kind::ExpectedTag, Position::new(0, 0))
        );
    }

    #[test]
    fn character_before_first_tag_after_whitespace() {
        let input = b"\n  foo#bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_err_eq!(
            tags.next(),
            Error::new(error::Kind::ExpectedTag, Position::new(1, 2))
        );
    }

    #[test]
    fn finds_single_tag() {
        let input = b"#foo:bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.next(), Tag::new(b"foo:bar;\n", Position::new(0, 0)));
    }

    #[test]
    fn finds_multiple_tag() {
        let input = b"#foo:bar;\n#baz;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.next(), Tag::new(b"foo:bar;\n", Position::new(0, 0)));
        assert_ok_eq!(tags.next(), Tag::new(b"baz;\n", Position::new(1, 0)));
    }

    #[test]
    fn finds_new_tag_without_previous_tag_ending() {
        let input = b"#foo:bar\n#baz;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.next(), Tag::new(b"foo:bar\n", Position::new(0, 0)));
        assert_ok_eq!(tags.next(), Tag::new(b"baz;\n", Position::new(1, 0)));
    }

    #[test]
    fn ignores_unescaped_number_sign() {
        let input = b"#foo:bar#baz;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(
            tags.next(),
            Tag::new(b"foo:bar#baz;\n", Position::new(0, 0))
        );
    }

    #[test]
    fn new_tag_on_same_line() {
        let input = b"#foo:bar;#baz;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.next(), Tag::new(b"foo:bar;", Position::new(0, 0)));
        assert_ok_eq!(tags.next(), Tag::new(b"baz;\n", Position::new(0, 9)));
    }

    #[test]
    fn escaped_number_sign() {
        let input = b"#foo:bar;\n\\#baz;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(
            tags.next(),
            Tag::new(b"foo:bar;\n\\#baz;\n", Position::new(0, 0))
        );
    }

    #[test]
    fn escaped_number_sign_without_previous_tag_ending() {
        let input = b"#foo:bar\n\\#baz;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(
            tags.next(),
            Tag::new(b"foo:bar\n\\#baz;\n", Position::new(0, 0))
        );
    }

    #[test]
    fn begins_with_comment() {
        let input = b"//comment\n#foo:bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.next(), Tag::new(b"foo:bar;\n", Position::new(1, 0)));
    }

    #[test]
    fn begins_with_single_slash() {
        let input = b"/#foo:bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_err_eq!(
            tags.next(),
            Error::new(error::Kind::ExpectedTag, Position::new(0, 0))
        );
    }

    #[test]
    fn tag_within_comment() {
        let input = b"//#foo:bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_err_eq!(
            tags.next(),
            Error::new(error::Kind::EndOfFile, Position::new(1, 0))
        );
    }

    #[test]
    fn comment_within_tag() {
        let input = b"#foo//comment;#\n:bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(
            tags.next(),
            Tag::new(b"foo//comment;#\n:bar;\n", Position::new(0, 0))
        );
    }

    #[test]
    fn tag_with_single_slash() {
        let input = b"#/;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.next(), Tag::new(b"/;\n", Position::new(0, 0)));
    }

    #[test]
    fn escaping_after_maybe_entering_comment() {
        let input = b"#/\\;;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.next(), Tag::new(b"/\\;;\n", Position::new(0, 0)));
    }

    #[test]
    fn normal_byte_after_maybe_entering_comment() {
        let input = b"#/foo;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.next(), Tag::new(b"/foo;\n", Position::new(0, 0)));
    }

    #[test]
    fn escaped_semicolon() {
        let input = b"#foo:bar\\;#baz;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(
            tags.next(),
            Tag::new(b"foo:bar\\;#baz;\n", Position::new(0, 0))
        );
    }

    #[test]
    fn has_next() {
        let input = b"#foo:bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.has_next(), true);
    }

    #[test]
    fn not_has_next() {
        let input = b"\n\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.has_next(), false);
    }

    #[test]
    fn has_next_then_query_next() {
        let input = b"#foo:bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.has_next(), true);
        assert_ok_eq!(tags.next(), Tag::new(b"foo:bar;\n", Position::new(0, 0)));
    }

    #[test]
    fn has_next_then_query_next_after_comment() {
        let input = b"//This is a comment# \n #foo:bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.has_next(), true);
        assert_ok_eq!(tags.next(), Tag::new(b"foo:bar;\n", Position::new(1, 1)));
    }

    #[test]
    fn has_next_character_before_first_tag() {
        let input = b"foo#bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_err_eq!(
            tags.has_next(),
            Error::new(error::Kind::ExpectedTag, Position::new(0, 0))
        );
    }

    #[test]
    fn has_next_slash_before_first_tag() {
        let input = b"/#bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_err_eq!(
            tags.has_next(),
            Error::new(error::Kind::ExpectedTag, Position::new(0, 0))
        );
    }

    #[test]
    fn has_next_repeats_error() {
        let input = b"foo#bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_err_eq!(
            tags.has_next(),
            Error::new(error::Kind::ExpectedTag, Position::new(0, 0))
        );
        assert_err_eq!(
            tags.has_next(),
            Error::new(error::Kind::ExpectedTag, Position::new(0, 0))
        );
    }

    #[test]
    fn revisit() {
        let input = b"#foo;\n";
        let mut tags = Tags::new(input.as_slice());

        let tag = assert_ok!(tags.next()).into_stored();
        unsafe { tags.revisit(tag) };
        assert_ok_eq!(tags.next(), Tag::new(b"foo;\n", Position::new(0, 0)));
    }

    #[test]
    fn exhausted() {
        let input = b"#foo;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok!(tags.next());

        assert_ok!(tags.assert_exhausted());
    }

    #[test]
    fn not_exhausted() {
        let input = b"#foo;\n#bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok!(tags.next());

        assert_err_eq!(
            tags.assert_exhausted(),
            Error::new(error::Kind::UnexpectedTag, Position::new(1, 0))
        );
    }

    #[test]
    fn not_exhausted_after_revisit() {
        let input = b"#foo;\n";
        let mut tags = Tags::new(input.as_slice());

        let tag = assert_ok!(tags.next()).into_stored();
        assert_ok!(tags.assert_exhausted());
        unsafe { tags.revisit(tag) };
        assert_err_eq!(
            tags.assert_exhausted(),
            Error::new(error::Kind::UnexpectedTag, Position::new(0, 0))
        );
    }

    #[test]
    fn current_position() {
        let input = b"#foo;\n";
        let tags = Tags::new(input.as_slice());

        assert_eq!(tags.current_position(), Position::new(0, 0));
    }

    #[test]
    fn current_position_after_iteration() {
        let input = b"#foo;\n#bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok!(tags.next());

        assert_eq!(tags.current_position(), Position::new(1, 0));
    }
}
