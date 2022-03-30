use std::marker::PhantomData;

enum State {
    None,
    Escaping,
    Escaped,
    EscapedDouble,
}

/// Escapes bytes, prepending `#`, `:`, `;`, `\`, and `//` characters with a `\`.
///
/// This should be used when preparing to write unknown values during serialization. to ensure the
/// serialization is correct.
pub(crate) struct Escaper<'a> {
    current_pointer: *const u8,
    end_pointer: *const u8,
    state: State,

    lifetime: PhantomData<&'a ()>,
}

impl<'a> Escaper<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        let pointer = bytes.as_ptr();
        Self {
            current_pointer: pointer,
            // SAFETY: Since `pointer` is the start of the `bytes` slice, adding the length of the
            // slice points directly past the slice, which is safe.
            end_pointer: unsafe { pointer.add(bytes.len()) },
            state: State::None,

            lifetime: PhantomData,
        }
    }
}

impl<'a> Iterator for Escaper<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pointer >= self.end_pointer {
            return None;
        }

        // SAFETY: We just verified that `current_pointer` is not past `end_pointer`, which
        // means it is a valid byte within the slice.
        let b = unsafe { *self.current_pointer };

        match self.state {
            State::None => {
                match (b, unsafe { self.current_pointer.add(1) } < self.end_pointer) {
                    (b'#' | b':' | b';' | b'\\', _) => {
                        self.state = State::Escaped;
                        Some(b'\\')
                    }
                    (b'/', true) => {
                        // SAFETY: The current pointer plus 1 is verified in this branch to be
                        // valid.
                        if unsafe { *self.current_pointer.add(1) } == b'/' {
                            self.state = State::EscapedDouble;
                            Some(b'\\')
                        } else {
                            // SAFETY: We verified above that current pointer was not at the end of its
                            // slice.
                            self.current_pointer = unsafe { self.current_pointer.add(1) };
                            Some(b)
                        }
                    }
                    _ => {
                        // SAFETY: We verified above that current pointer was not at the end of its
                        // slice.
                        self.current_pointer = unsafe { self.current_pointer.add(1) };
                        Some(b)
                    }
                }
            }
            State::Escaping => {
                self.state = State::Escaped;
                Some(b'\\')
            }
            State::Escaped => {
                self.state = State::None;
                // SAFETY: We verified above that current pointer was not at the end of its
                // slice.
                self.current_pointer = unsafe { self.current_pointer.add(1) };
                Some(b)
            }
            State::EscapedDouble => {
                self.state = State::Escaping;
                // SAFETY: We verified above that current pointer was not at the end of its
                // slice.
                self.current_pointer = unsafe { self.current_pointer.add(1) };
                Some(b)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Escaper;

    #[test]
    fn empty() {
        let escaper = Escaper::new(b"");

        assert_eq!(escaper.collect::<Vec<_>>(), b"");
    }

    #[test]
    fn no_escapes() {
        let escaper = Escaper::new(b"foo");

        assert_eq!(escaper.collect::<Vec<_>>(), b"foo");
    }

    #[test]
    fn escapes_number_sign() {
        let escaper = Escaper::new(b"#");

        assert_eq!(escaper.collect::<Vec<_>>(), b"\\#");
    }

    #[test]
    fn escapes_colon() {
        let escaper = Escaper::new(b":");

        assert_eq!(escaper.collect::<Vec<_>>(), b"\\:");
    }

    #[test]
    fn escapes_semicolon() {
        let escaper = Escaper::new(b";");

        assert_eq!(escaper.collect::<Vec<_>>(), b"\\;");
    }

    #[test]
    fn escapes_backslash() {
        let escaper = Escaper::new(b"\\");

        assert_eq!(escaper.collect::<Vec<_>>(), b"\\\\");
    }

    #[test]
    fn no_escape_single_forward_slash() {
        let escaper = Escaper::new(b"/foo");

        assert_eq!(escaper.collect::<Vec<_>>(), b"/foo");
    }

    #[test]
    fn no_escape_single_forward_slash_at_end() {
        let escaper = Escaper::new(b"/");

        assert_eq!(escaper.collect::<Vec<_>>(), b"/");
    }

    #[test]
    fn escapes_double_forward_slash() {
        let escaper = Escaper::new(b"//");

        assert_eq!(escaper.collect::<Vec<_>>(), b"\\/\\/");
    }

    #[test]
    fn escapes_triple_forward_slash() {
        let escaper = Escaper::new(b"///");

        assert_eq!(escaper.collect::<Vec<_>>(), b"\\/\\//");
    }

    #[test]
    fn escapes_multiple() {
        let escaper = Escaper::new(b"foo//bar#baz;qux:quux\\");

        assert_eq!(
            escaper.collect::<Vec<_>>(),
            b"foo\\/\\/bar\\#baz\\;qux\\:quux\\\\"
        );
    }

    #[test]
    fn escapes_back_to_back() {
        let escaper = Escaper::new(b"#:;\\////");

        assert_eq!(escaper.collect::<Vec<_>>(), b"\\#\\:\\;\\\\\\/\\/\\/\\/");
    }
}
