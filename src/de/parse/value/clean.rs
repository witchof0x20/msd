/// Cleans values, removing comments and interpreting escape sequences.

#[derive(Clone)]
pub(super) struct Clean<'a> {
    bytes: &'a [u8],
}

impl<'a> Clean<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }
}

impl<'a> Iterator for Clean<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let byte = *self.bytes.get(0)?;
        // SAFETY: If `self.bytes` is empty, it will have returned in the previous statement.
        self.bytes = unsafe { self.bytes.get_unchecked(1..) };

        match byte {
            b'\\' => {
                // Possibly escaping the next byte.
                match self.bytes.get(0).copied() {
                    Some(next_byte) => match next_byte {
                        b':' | b';' | b'\\' | b'/' | b'#' => {
                            // Escape the character.
                            // SAFETY: We can only be in this branch is self.bytes is nonempty.
                            self.bytes = unsafe { self.bytes.get_unchecked(1..) };
                            Some(next_byte)
                        }
                        _ => Some(byte),
                    },
                    None => Some(byte),
                }
            }
            b'/' => {
                // Possibly entering a comment.
                match self.bytes.get(0).copied() {
                    Some(b'/') => {
                        // Inside a comment.
                        loop {
                            let comment_byte = *self.bytes.get(0)?;
                            // SAFETY: If `self.bytes` is empty, it will have returned in the
                            // previous statement.
                            self.bytes = unsafe { self.bytes.get_unchecked(1..) };
                            if matches!(comment_byte, b'\n') {
                                return Some(comment_byte);
                            }
                        }
                    }
                    _ => Some(byte),
                }
            }
            _ => Some(byte),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Clean;

    #[test]
    fn empty() {
        assert_eq!(Clean::new(b"").collect::<Vec<_>>(), b"");
    }

    #[test]
    fn plain() {
        assert_eq!(Clean::new(b"foo").collect::<Vec<_>>(), b"foo");
    }

    #[test]
    fn escaped() {
        assert_eq!(
            Clean::new(b"\\/foo\\\\bar\\#baz\\;qux\\:").collect::<Vec<_>>(),
            b"/foo\\bar#baz;qux:"
        );
    }

    #[test]
    fn not_escaped() {
        assert_eq!(Clean::new(b"foo\\bar").collect::<Vec<_>>(), b"foo\\bar");
    }

    #[test]
    fn backslash_at_end() {
        assert_eq!(Clean::new(b"foo\\").collect::<Vec<_>>(), b"foo\\");
    }

    #[test]
    fn comment() {
        assert_eq!(
            Clean::new(b"foo // bar\nbaz").collect::<Vec<_>>(),
            b"foo \nbaz"
        );
    }

    #[test]
    fn escaped_comment() {
        assert_eq!(
            Clean::new(b"foo /\\/ bar\nbaz").collect::<Vec<_>>(),
            b"foo // bar\nbaz"
        );
    }

    #[test]
    fn forward_slash_by_itself() {
        assert_eq!(Clean::new(b"foo/bar").collect::<Vec<_>>(), b"foo/bar");
    }

    #[test]
    fn forward_slash_at_end() {
        assert_eq!(Clean::new(b"foo/").collect::<Vec<_>>(), b"foo/");
    }
}
