use crate::de::{error, Error, Result};

fn trim_ascii_whitespace(bytes: &[u8]) -> &[u8] {
    // SAFETY:
    // 1) Both ends of the range are found by .position and .rposition when iterating over the
    // slice. Therefore they are guaranteed to be within the bounds of the slice.
    // 2) Since .position() returns early when no position is found, we know that .rposition() will
    // always find a value.
    unsafe {
        bytes.get_unchecked(
            match bytes.iter().position(|byte| !byte.is_ascii_whitespace()) {
                Some(index) => index,
                None => return &[],
            }
                ..=bytes
                    .iter()
                    .rposition(|byte| !byte.is_ascii_whitespace())
                    .unwrap_unchecked(),
        )
    }
}

#[derive(Debug, PartialEq)]
pub(in crate::de) struct Value<'a> {
    bytes: &'a [u8],
    line: usize,
    column: usize,
}

impl<'a> Value<'a> {
    pub(super) fn new(bytes: &'a [u8], line: usize, column: usize) -> Self {
        Self {
            bytes,
            line,
            column,
        }
    }

    pub(in crate::de) fn parse_bool(&self) -> Result<bool> {
        match trim_ascii_whitespace(self.bytes) {
            b"true" => Ok(true),
            b"false" => Ok(false),
            _ => Err(Error::new(
                error::Kind::ExpectedBool,
                self.line,
                self.column,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{trim_ascii_whitespace, Value};
    use crate::de::{error, Error};
    use claim::{assert_err_eq, assert_ok_eq};

    #[test]
    fn trims_ascii_whitespace() {
        assert_eq!(trim_ascii_whitespace(b"  \tfoo \n"), b"foo");
    }

    #[test]
    fn trims_ascii_whitespace_from_front() {
        assert_eq!(trim_ascii_whitespace(b"  \tfoo"), b"foo");
    }

    #[test]
    fn trims_ascii_whitespace_from_back() {
        assert_eq!(trim_ascii_whitespace(b"foo \n"), b"foo");
    }

    #[test]
    fn trims_ascii_whitespace_with_whitespace_in_middle() {
        assert_eq!(trim_ascii_whitespace(b"  \tfoo  bar \n"), b"foo  bar");
    }

    #[test]
    fn trims_ascii_whitespace_to_empty() {
        assert_eq!(trim_ascii_whitespace(b"  \t \n"), b"");
    }

    #[test]
    fn value_parse_bool_true() {
        let value = Value::new(b"true", 0, 0);

        assert_ok_eq!(value.parse_bool(), true);
    }

    #[test]
    fn value_parse_bool_false() {
        let value = Value::new(b"false", 0, 0);

        assert_ok_eq!(value.parse_bool(), false);
    }

    #[test]
    fn value_parse_bool_invalid() {
        let value = Value::new(b"not a bool", 0, 0);

        assert_err_eq!(
            value.parse_bool(),
            Error::new(error::Kind::ExpectedBool, 0, 0)
        );
    }
}
