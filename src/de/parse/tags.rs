use super::Tag;
use crate::de::{error, parse::StoredTag, Error, Result};
use std::io::{Bytes, Read};

struct Position {
    line: usize,
    column: usize,
}

enum TagState {
    InTag,
    MaybeEndingTag,
    None,
}

enum NewlineState {
    StartingNewline,
    None,
}

enum EscapingState {
    Escaping,
    None,
}

enum CommentState {
    MaybeEnteringComment,
    InComment,
    None,
}

pub(in crate::de) struct Tags<R> {
    reader: Bytes<R>,

    buffer: Vec<u8>,
    started_line: usize,
    started_column: usize,

    tag_state: TagState,
    newline_state: NewlineState,
    escaping_state: EscapingState,
    comment_state: CommentState,

    current_line: usize,
    current_column: usize,

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
            started_line: 0,
            started_column: 0,

            tag_state: TagState::None,
            newline_state: NewlineState::StartingNewline,
            escaping_state: EscapingState::None,
            comment_state: CommentState::None,

            current_line: 0,
            current_column: 0,

            encountered_error: None,
            exhausted: false,

            revisit: None,
        }
    }

    /// Returns the next tag in the input, if there is one.
    ///
    /// Note that this is not an `Iterator::next()`, because it is impossible for an iterator to
    /// return items that have a shorter lifetime than the iterator itself. Each `Tag` returned
    /// here only lives until the next call to `next()`, because it borrows from a reused internal
    /// buffer.
    pub(in crate::de) fn next<'buffer>(&'buffer mut self) -> Result<Tag<'buffer>> {
        if let Some(error) = self.encountered_error {
            return Err(error);
        }

        if let Some(revisit) = self.revisit.take() {
            return Ok(
                // `revisit` is guaranteed to point to valid contents on the currently buffer.
                unsafe { revisit.into_tag() },
            );
        }

        // Reuse the same buffer.
        self.buffer.clear();
        // Setting this value to a `Some(Position)` causes the function to return at the end of the
        // current iteration. In other words, it indicates that the end of a tag has been reached.
        let mut tag_position = None;

        loop {
            let byte = match self.reader.next() {
                Some(byte) => match byte {
                    Ok(byte) => byte,
                    Err(_error) => {
                        let error =
                            Error::new(error::Kind::Io, self.current_line, self.current_column);
                        self.encountered_error = Some(error);
                        self.exhausted = true;
                        return Err(error);
                    }
                },
                None => {
                    self.exhausted = true;
                    if self.buffer.is_empty() {
                        let error = Error::new(
                            error::Kind::EndOfFile,
                            self.current_line,
                            self.current_column,
                        );
                        self.encountered_error = Some(error);
                        return Err(error);
                    } else {
                        return Ok(Tag::new(
                            &self.buffer,
                            self.started_line,
                            self.started_column,
                        ));
                    }
                }
            };

            // Process byte.
            if matches!(self.comment_state, CommentState::InComment) {
                // Consume bytes until we are on a new line.
                if matches!(byte, b'\n') {
                    self.comment_state = CommentState::None;
                }
                if matches!(self.tag_state, TagState::InTag | TagState::MaybeEndingTag) {
                    self.buffer.push(byte);
                }
            } else {
                match self.tag_state {
                    TagState::InTag => {
                        if matches!(self.escaping_state, EscapingState::None) {
                            match byte {
                                b'#' => {
                                    // We are lenient on the formatting here. If a `#` is at the
                                    // start of a newline we begin a new tag and assume the
                                    // previous tag was missing the closing `;` (some old
                                    // specifications of MSD didn't explicitly require the `;`). If
                                    // we are in the middle of a line, we assume it was meant to be
                                    // escaped.
                                    if matches!(self.newline_state, NewlineState::StartingNewline) {
                                        // Entering a new tag. Return the previous one.
                                        tag_position = Some(Position {
                                            line: self.started_line,
                                            column: self.started_column,
                                        });
                                        self.started_line = self.current_line;
                                        self.started_column = self.current_column;
                                        self.tag_state = TagState::InTag;
                                    } else {
                                        self.buffer.push(byte);
                                    }
                                }
                                b';' => {
                                    self.tag_state = TagState::MaybeEndingTag;
                                    self.buffer.push(byte);
                                }
                                b'\\' => {
                                    self.escaping_state = EscapingState::Escaping;
                                    self.buffer.push(byte);
                                }
                                b'/' => {
                                    if matches!(
                                        self.comment_state,
                                        CommentState::MaybeEnteringComment
                                    ) {
                                        self.comment_state = CommentState::InComment;
                                    } else {
                                        self.comment_state = CommentState::MaybeEnteringComment;
                                    }
                                    self.buffer.push(byte);
                                }
                                _ => {
                                    self.buffer.push(byte);
                                }
                            }
                        } else {
                            // Escaping the current character.
                            self.buffer.push(byte);
                            self.escaping_state = EscapingState::None;
                        }
                    }
                    TagState::MaybeEndingTag => {
                        match byte {
                            b'#' => {
                                // Entering a new tag. Return the previous one.
                                tag_position = Some(Position {
                                    line: self.started_line,
                                    column: self.started_column,
                                });
                                self.started_line = self.current_line;
                                self.started_column = self.current_column;
                                self.tag_state = TagState::InTag;
                            }
                            b'\\' => {
                                // Escaping the next character.
                                self.tag_state = TagState::InTag;
                                self.escaping_state = EscapingState::Escaping;
                                self.buffer.push(byte);
                            }
                            b'/' => {
                                // Possibly entering a comment.
                                self.tag_state = TagState::InTag;
                                self.comment_state = CommentState::MaybeEnteringComment;
                                self.buffer.push(byte);
                            }
                            _ => {
                                if !byte.is_ascii_whitespace() {
                                    // Still in the tag.
                                    self.tag_state = TagState::InTag;
                                }
                                self.buffer.push(byte);
                            }
                        }
                    }
                    TagState::None => {
                        match byte {
                            b'#' => {
                                if matches!(self.comment_state, CommentState::MaybeEnteringComment)
                                {
                                    let error = Error::new(
                                        error::Kind::ExpectedTag,
                                        self.current_line,
                                        self.current_column - 1,
                                    );
                                    self.encountered_error = Some(error);
                                    return Err(error);
                                }
                                self.tag_state = TagState::InTag;
                                self.started_line = self.current_line;
                                self.started_column = self.current_column;
                            }
                            b'/' => {
                                if matches!(self.comment_state, CommentState::MaybeEnteringComment)
                                {
                                    self.comment_state = CommentState::InComment;
                                } else {
                                    self.comment_state = CommentState::MaybeEnteringComment;
                                }
                            }
                            _ => {
                                if matches!(self.comment_state, CommentState::MaybeEnteringComment)
                                {
                                    let error = Error::new(
                                        error::Kind::ExpectedTag,
                                        self.current_line,
                                        self.current_column - 1,
                                    );
                                    self.encountered_error = Some(error);
                                    return Err(error);
                                }
                                // Non-whitespace bytes are not allowed before the first tag.
                                if !byte.is_ascii_whitespace() {
                                    let error = Error::new(
                                        error::Kind::ExpectedTag,
                                        self.current_line,
                                        self.current_column,
                                    );
                                    self.encountered_error = Some(error);
                                    return Err(error);
                                }
                            }
                        }
                    }
                }
            }

            if matches!(byte, b'\n') {
                self.newline_state = NewlineState::StartingNewline;
            } else {
                self.newline_state = NewlineState::None;
            }

            if matches!(self.newline_state, NewlineState::StartingNewline) {
                self.current_line += 1;
                self.current_column = 0;
            } else {
                self.current_column += 1;
            }

            if let Some(position) = tag_position {
                return Ok(Tag::new(&self.buffer, position.line, position.column));
            }
        }
    }

    /// Returns whether there will be another tag.
    ///
    /// If this returns `true`, then a call to `next()` will return `Some(tag)`.
    pub(in crate::de) fn has_next(&mut self) -> Result<bool> {
        // The iterator will only be in the state below if no tags have been returned yet.
        // Simply find the first tag if it exists.
        if matches!(self.tag_state, TagState::None) {
            loop {
                let byte = match self.reader.next() {
                    Some(byte) => match byte {
                        Ok(byte) => byte,
                        Err(_error) => {
                            let error =
                                Error::new(error::Kind::Io, self.current_line, self.current_column);
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
                if matches!(self.comment_state, CommentState::InComment) {
                    // Consume bytes until we are on a new line.
                    if matches!(byte, b'\n') {
                        self.comment_state = CommentState::None;
                    }
                } else {
                    match byte {
                        b'#' => {
                            if matches!(self.comment_state, CommentState::MaybeEnteringComment) {
                                let error = Error::new(
                                    error::Kind::ExpectedTag,
                                    self.current_line,
                                    self.current_column - 1,
                                );
                                self.encountered_error = Some(error);
                                break;
                            }
                            self.tag_state = TagState::InTag;
                            self.started_line = self.current_line;
                            self.started_column = self.current_column;
                            break;
                        }
                        b'/' => {
                            if matches!(self.comment_state, CommentState::MaybeEnteringComment) {
                                self.comment_state = CommentState::InComment;
                            } else {
                                self.comment_state = CommentState::MaybeEnteringComment;
                            }
                        }
                        _ => {
                            if matches!(self.comment_state, CommentState::MaybeEnteringComment) {
                                let error = Error::new(
                                    error::Kind::ExpectedTag,
                                    self.current_line,
                                    self.current_column - 1,
                                );
                                self.encountered_error = Some(error);
                                break;
                            }
                            // Non-whitespace bytes are not allowed before the first tag.
                            if !byte.is_ascii_whitespace() {
                                let error = Error::new(
                                    error::Kind::ExpectedTag,
                                    self.current_line,
                                    self.current_column,
                                );
                                self.encountered_error = Some(error);
                                break;
                            }
                        }
                    }
                }

                if matches!(byte, b'\n') {
                    self.newline_state = NewlineState::StartingNewline;
                } else {
                    self.newline_state = NewlineState::None;
                }

                if matches!(self.newline_state, NewlineState::StartingNewline) {
                    self.current_line += 1;
                    self.current_column = 0;
                } else {
                    self.current_column += 1;
                }
            }
        }

        if let Some(err) = self.encountered_error {
            Err(err)
        } else {
            Ok(!self.exhausted
                && matches!(self.tag_state, TagState::InTag | TagState::MaybeEndingTag))
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
                revisit.origin_line(),
                revisit.origin_column(),
            ))
        } else if self.exhausted {
            Ok(())
        } else {
            Err(Error::new(
                error::Kind::UnexpectedTag,
                self.started_line,
                self.started_column,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tags;
    use crate::de::{error, parse::Tag, Error};
    use claim::{assert_err_eq, assert_ok, assert_ok_eq};

    #[test]
    fn empty_reader() {
        let input = b"";
        let mut tags = Tags::new(input.as_slice());

        assert_err_eq!(tags.next(), Error::new(error::Kind::EndOfFile, 0, 0));
    }

    #[test]
    fn character_before_first_tag() {
        let input = b"foo#bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_err_eq!(tags.next(), Error::new(error::Kind::ExpectedTag, 0, 0));
    }

    #[test]
    fn character_before_first_tag_after_whitespace() {
        let input = b"\n  foo#bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_err_eq!(tags.next(), Error::new(error::Kind::ExpectedTag, 1, 2));
    }

    #[test]
    fn finds_single_tag() {
        let input = b"#foo:bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.next(), Tag::new(b"foo:bar;\n", 0, 0,));
    }

    #[test]
    fn finds_multiple_tag() {
        let input = b"#foo:bar;\n#baz;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.next(), Tag::new(b"foo:bar;\n", 0, 0,));
        assert_ok_eq!(tags.next(), Tag::new(b"baz;\n", 1, 0,));
    }

    #[test]
    fn finds_new_tag_without_previous_tag_ending() {
        let input = b"#foo:bar\n#baz;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.next(), Tag::new(b"foo:bar\n", 0, 0,));
        assert_ok_eq!(tags.next(), Tag::new(b"baz;\n", 1, 0,));
    }

    #[test]
    fn ignores_unescaped_number_sign() {
        let input = b"#foo:bar#baz;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.next(), Tag::new(b"foo:bar#baz;\n", 0, 0,));
    }

    #[test]
    fn new_tag_on_same_line() {
        let input = b"#foo:bar;#baz;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.next(), Tag::new(b"foo:bar;", 0, 0,));
        assert_ok_eq!(tags.next(), Tag::new(b"baz;\n", 0, 9,));
    }

    #[test]
    fn escaped_number_sign() {
        let input = b"#foo:bar;\n\\#baz;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.next(), Tag::new(b"foo:bar;\n\\#baz;\n", 0, 0,));
    }

    #[test]
    fn escaped_number_sign_without_previous_tag_ending() {
        let input = b"#foo:bar\n\\#baz;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.next(), Tag::new(b"foo:bar\n\\#baz;\n", 0, 0,));
    }

    #[test]
    fn begins_with_comment() {
        let input = b"//comment\n#foo:bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.next(), Tag::new(b"foo:bar;\n", 1, 0,));
    }

    #[test]
    fn begins_with_single_slash() {
        let input = b"/#foo:bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_err_eq!(tags.next(), Error::new(error::Kind::ExpectedTag, 0, 0));
    }

    #[test]
    fn tag_within_comment() {
        let input = b"//#foo:bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_err_eq!(tags.next(), Error::new(error::Kind::EndOfFile, 1, 0));
    }

    #[test]
    fn escaped_semicolon() {
        let input = b"#foo:bar\\;#baz;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.next(), Tag::new(b"foo:bar\\;#baz;\n", 0, 0,));
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
        assert_ok_eq!(tags.next(), Tag::new(b"foo:bar;\n", 0, 0));
    }

    #[test]
    fn has_next_then_query_next_after_comment() {
        let input = b"//This is a comment# \n #foo:bar;\n";
        let mut tags = Tags::new(input.as_slice());

        assert_ok_eq!(tags.has_next(), true);
        assert_ok_eq!(tags.next(), Tag::new(b"foo:bar;\n", 1, 1));
    }

    #[test]
    fn revisit() {
        let input = b"#foo;\n";
        let mut tags = Tags::new(input.as_slice());

        let tag = assert_ok!(tags.next()).into_stored();
        unsafe { tags.revisit(tag) };
        assert_ok_eq!(tags.next(), Tag::new(b"foo;\n", 0, 0));
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
            Error::new(error::Kind::UnexpectedTag, 1, 0)
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
            Error::new(error::Kind::UnexpectedTag, 0, 0)
        );
    }
}
