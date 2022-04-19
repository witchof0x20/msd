enum State {
    TrimmingBeginning,
    ReturningBytes,
    IntermediateWhitespace,
    Completed,
}

pub(super) struct Trim<I> {
    iter: I,
    state: State,
}

impl<I> Trim<I> {
    pub(super) fn new(iter: I) -> Self {
        Self {
            iter,
            state: State::TrimmingBeginning,
        }
    }
}

impl<I> Iterator for Trim<I>
where
    I: Iterator<Item = u8> + Clone,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::TrimmingBeginning => loop {
                let byte = self.iter.next()?;

                if !byte.is_ascii_whitespace() {
                    self.state = State::ReturningBytes;
                    return Some(byte);
                }
            },
            State::ReturningBytes => {
                let byte = self.iter.next()?;

                if byte.is_ascii_whitespace() {
                    match self.iter.clone().find(|b| !b.is_ascii_whitespace()) {
                        Some(_) => {
                            self.state = State::IntermediateWhitespace;
                            Some(byte)
                        }
                        None => {
                            self.state = State::Completed;
                            None
                        }
                    }
                } else {
                    Some(byte)
                }
            }
            State::IntermediateWhitespace => {
                let byte = self.iter.next()?;
                if !byte.is_ascii_whitespace() {
                    self.state = State::ReturningBytes;
                }
                Some(byte)
            }
            State::Completed => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Trim;

    #[test]
    fn empty() {
        assert_eq!(Trim::new(b"".iter().copied()).collect::<Vec<_>>(), b"");
    }

    #[test]
    fn nothing_to_trim() {
        assert_eq!(
            Trim::new(b"foo".iter().copied()).collect::<Vec<_>>(),
            b"foo"
        );
    }

    #[test]
    fn trims_front() {
        assert_eq!(
            Trim::new(b" \t\nfoo".iter().copied()).collect::<Vec<_>>(),
            b"foo"
        );
    }

    #[test]
    fn trims_back() {
        assert_eq!(
            Trim::new(b"foo\n \t".iter().copied()).collect::<Vec<_>>(),
            b"foo"
        );
    }

    #[test]
    fn trims_front_and_back() {
        assert_eq!(
            Trim::new(b" \t\nfoo\n \t".iter().copied()).collect::<Vec<_>>(),
            b"foo"
        );
    }

    #[test]
    fn trims_with_whitespace_in_middle() {
        assert_eq!(
            Trim::new(b" \t\nfoo  bar\n \t".iter().copied()).collect::<Vec<_>>(),
            b"foo  bar"
        );
    }

    #[test]
    fn trims_to_empty() {
        assert_eq!(
            Trim::new(b" \t\n  \n".iter().copied()).collect::<Vec<_>>(),
            b""
        );
    }
}
