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

    pub(in crate::de) fn parse_i8(&self) -> Result<i8> {
        lexical_core::parse(trim_ascii_whitespace(self.bytes)).or(Err(Error::new(
            error::Kind::ExpectedI8,
            self.line,
            self.column,
        )))
    }

    pub(in crate::de) fn parse_i16(&self) -> Result<i16> {
        lexical_core::parse(trim_ascii_whitespace(self.bytes)).or(Err(Error::new(
            error::Kind::ExpectedI16,
            self.line,
            self.column,
        )))
    }

    pub(in crate::de) fn parse_i32(&self) -> Result<i32> {
        lexical_core::parse(trim_ascii_whitespace(self.bytes)).or(Err(Error::new(
            error::Kind::ExpectedI32,
            self.line,
            self.column,
        )))
    }

    pub(in crate::de) fn parse_i64(&self) -> Result<i64> {
        lexical_core::parse(trim_ascii_whitespace(self.bytes)).or(Err(Error::new(
            error::Kind::ExpectedI64,
            self.line,
            self.column,
        )))
    }

    pub(in crate::de) fn parse_i128(&self) -> Result<i128> {
        lexical_core::parse(trim_ascii_whitespace(self.bytes)).or(Err(Error::new(
            error::Kind::ExpectedI128,
            self.line,
            self.column,
        )))
    }

    pub(in crate::de) fn parse_u8(&self) -> Result<u8> {
        lexical_core::parse(trim_ascii_whitespace(self.bytes)).or(Err(Error::new(
            error::Kind::ExpectedU8,
            self.line,
            self.column,
        )))
    }

    pub(in crate::de) fn parse_u16(&self) -> Result<u16> {
        lexical_core::parse(trim_ascii_whitespace(self.bytes)).or(Err(Error::new(
            error::Kind::ExpectedU16,
            self.line,
            self.column,
        )))
    }

    pub(in crate::de) fn parse_u32(&self) -> Result<u32> {
        lexical_core::parse(trim_ascii_whitespace(self.bytes)).or(Err(Error::new(
            error::Kind::ExpectedU32,
            self.line,
            self.column,
        )))
    }

    pub(in crate::de) fn parse_u64(&self) -> Result<u64> {
        lexical_core::parse(trim_ascii_whitespace(self.bytes)).or(Err(Error::new(
            error::Kind::ExpectedU64,
            self.line,
            self.column,
        )))
    }

    pub(in crate::de) fn parse_u128(&self) -> Result<u128> {
        lexical_core::parse(trim_ascii_whitespace(self.bytes)).or(Err(Error::new(
            error::Kind::ExpectedU128,
            self.line,
            self.column,
        )))
    }

    pub(in crate::de) fn parse_f32(&self) -> Result<f32> {
        lexical_core::parse(trim_ascii_whitespace(self.bytes)).or(Err(Error::new(
            error::Kind::ExpectedF32,
            self.line,
            self.column,
        )))
    }

    pub(in crate::de) fn parse_f64(&self) -> Result<f64> {
        lexical_core::parse(trim_ascii_whitespace(self.bytes)).or(Err(Error::new(
            error::Kind::ExpectedF64,
            self.line,
            self.column,
        )))
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

    #[test]
    fn value_parse_i8_positive() {
        let value = Value::new(b"42", 0, 0);

        assert_ok_eq!(value.parse_i8(), 42);
    }

    #[test]
    fn value_parse_i8_negative() {
        let value = Value::new(b"-42", 0, 0);

        assert_ok_eq!(value.parse_i8(), -42);
    }

    #[test]
    fn value_parse_i8_zero() {
        let value = Value::new(b"0", 0, 0);

        assert_ok_eq!(value.parse_i8(), 0);
    }

    #[test]
    fn value_parse_i8_positive_overflow() {
        let value = Value::new(b"128", 0, 0);

        assert_err_eq!(value.parse_i8(), Error::new(error::Kind::ExpectedI8, 0, 0));
    }

    #[test]
    fn value_parse_i8_negative_overflow() {
        let value = Value::new(b"-129", 0, 0);

        assert_err_eq!(value.parse_i8(), Error::new(error::Kind::ExpectedI8, 0, 0));
    }

    #[test]
    fn value_parse_i8_invalid() {
        let value = Value::new(b"invalid", 0, 0);

        assert_err_eq!(value.parse_i8(), Error::new(error::Kind::ExpectedI8, 0, 0));
    }

    #[test]
    fn value_parse_i8_whitespace() {
        let value = Value::new(b"  42 \n", 0, 0);

        assert_ok_eq!(value.parse_i8(), 42);
    }

    #[test]
    fn value_parse_i16_positive() {
        let value = Value::new(b"42", 0, 0);

        assert_ok_eq!(value.parse_i16(), 42);
    }

    #[test]
    fn value_parse_i16_negative() {
        let value = Value::new(b"-42", 0, 0);

        assert_ok_eq!(value.parse_i16(), -42);
    }

    #[test]
    fn value_parse_i16_zero() {
        let value = Value::new(b"0", 0, 0);

        assert_ok_eq!(value.parse_i16(), 0);
    }

    #[test]
    fn value_parse_i16_positive_overflow() {
        let value = Value::new(b"32768", 0, 0);

        assert_err_eq!(
            value.parse_i16(),
            Error::new(error::Kind::ExpectedI16, 0, 0)
        );
    }

    #[test]
    fn value_parse_i16_negative_overflow() {
        let value = Value::new(b"-32769", 0, 0);

        assert_err_eq!(
            value.parse_i16(),
            Error::new(error::Kind::ExpectedI16, 0, 0)
        );
    }

    #[test]
    fn value_parse_i16_invalid() {
        let value = Value::new(b"invalid", 0, 0);

        assert_err_eq!(
            value.parse_i16(),
            Error::new(error::Kind::ExpectedI16, 0, 0)
        );
    }

    #[test]
    fn value_parse_i16_whitespace() {
        let value = Value::new(b"  42 \n", 0, 0);

        assert_ok_eq!(value.parse_i16(), 42);
    }

    #[test]
    fn value_parse_i32_positive() {
        let value = Value::new(b"42", 0, 0);

        assert_ok_eq!(value.parse_i32(), 42);
    }

    #[test]
    fn value_parse_i32_negative() {
        let value = Value::new(b"-42", 0, 0);

        assert_ok_eq!(value.parse_i32(), -42);
    }

    #[test]
    fn value_parse_i32_zero() {
        let value = Value::new(b"0", 0, 0);

        assert_ok_eq!(value.parse_i32(), 0);
    }

    #[test]
    fn value_parse_i32_positive_overflow() {
        let value = Value::new(b"2147483648", 0, 0);

        assert_err_eq!(
            value.parse_i32(),
            Error::new(error::Kind::ExpectedI32, 0, 0)
        );
    }

    #[test]
    fn value_parse_i32_negative_overflow() {
        let value = Value::new(b"-2147483649", 0, 0);

        assert_err_eq!(
            value.parse_i32(),
            Error::new(error::Kind::ExpectedI32, 0, 0)
        );
    }

    #[test]
    fn value_parse_i32_invalid() {
        let value = Value::new(b"invalid", 0, 0);

        assert_err_eq!(
            value.parse_i32(),
            Error::new(error::Kind::ExpectedI32, 0, 0)
        );
    }

    #[test]
    fn value_parse_i32_whitespace() {
        let value = Value::new(b"  42 \n", 0, 0);

        assert_ok_eq!(value.parse_i32(), 42);
    }

    #[test]
    fn value_parse_i64_positive() {
        let value = Value::new(b"42", 0, 0);

        assert_ok_eq!(value.parse_i64(), 42);
    }

    #[test]
    fn value_parse_i64_negative() {
        let value = Value::new(b"-42", 0, 0);

        assert_ok_eq!(value.parse_i64(), -42);
    }

    #[test]
    fn value_parse_i64_zero() {
        let value = Value::new(b"0", 0, 0);

        assert_ok_eq!(value.parse_i64(), 0);
    }

    #[test]
    fn value_parse_i64_positive_overflow() {
        let value = Value::new(b"9223372036854775808", 0, 0);

        assert_err_eq!(
            value.parse_i64(),
            Error::new(error::Kind::ExpectedI64, 0, 0)
        );
    }

    #[test]
    fn value_parse_i64_negative_overflow() {
        let value = Value::new(b"-9223372036854775809", 0, 0);

        assert_err_eq!(
            value.parse_i64(),
            Error::new(error::Kind::ExpectedI64, 0, 0)
        );
    }

    #[test]
    fn value_parse_i64_invalid() {
        let value = Value::new(b"invalid", 0, 0);

        assert_err_eq!(
            value.parse_i64(),
            Error::new(error::Kind::ExpectedI64, 0, 0)
        );
    }

    #[test]
    fn value_parse_i64_whitespace() {
        let value = Value::new(b"  42 \n", 0, 0);

        assert_ok_eq!(value.parse_i64(), 42);
    }

    #[test]
    fn value_parse_i128_positive() {
        let value = Value::new(b"42", 0, 0);

        assert_ok_eq!(value.parse_i128(), 42);
    }

    #[test]
    fn value_parse_i128_negative() {
        let value = Value::new(b"-42", 0, 0);

        assert_ok_eq!(value.parse_i128(), -42);
    }

    #[test]
    fn value_parse_i128_zero() {
        let value = Value::new(b"0", 0, 0);

        assert_ok_eq!(value.parse_i128(), 0);
    }

    #[test]
    fn value_parse_i128_positive_overflow() {
        let value = Value::new(b"170141183460469231731687303715884105728", 0, 0);

        assert_err_eq!(
            value.parse_i128(),
            Error::new(error::Kind::ExpectedI128, 0, 0)
        );
    }

    #[test]
    fn value_parse_i128_negative_overflow() {
        let value = Value::new(b"-170141183460469231731687303715884105729", 0, 0);

        assert_err_eq!(
            value.parse_i128(),
            Error::new(error::Kind::ExpectedI128, 0, 0)
        );
    }

    #[test]
    fn value_parse_i128_invalid() {
        let value = Value::new(b"invalid", 0, 0);

        assert_err_eq!(
            value.parse_i128(),
            Error::new(error::Kind::ExpectedI128, 0, 0)
        );
    }

    #[test]
    fn value_parse_i128_whitespace() {
        let value = Value::new(b"  42 \n", 0, 0);

        assert_ok_eq!(value.parse_i128(), 42);
    }

    #[test]
    fn value_parse_u8() {
        let value = Value::new(b"42", 0, 0);

        assert_ok_eq!(value.parse_u8(), 42);
    }

    #[test]
    fn value_parse_u8_zero() {
        let value = Value::new(b"0", 0, 0);

        assert_ok_eq!(value.parse_u8(), 0);
    }

    #[test]
    fn value_parse_u8_overflow() {
        let value = Value::new(b"256", 0, 0);

        assert_err_eq!(value.parse_u8(), Error::new(error::Kind::ExpectedU8, 0, 0));
    }

    #[test]
    fn value_parse_u8_invalid() {
        let value = Value::new(b"invalid", 0, 0);

        assert_err_eq!(value.parse_u8(), Error::new(error::Kind::ExpectedU8, 0, 0));
    }

    #[test]
    fn value_parse_u8_whitespace() {
        let value = Value::new(b"  42 \n", 0, 0);

        assert_ok_eq!(value.parse_u8(), 42);
    }

    #[test]
    fn value_parse_u16() {
        let value = Value::new(b"42", 0, 0);

        assert_ok_eq!(value.parse_u16(), 42);
    }

    #[test]
    fn value_parse_u16_zero() {
        let value = Value::new(b"0", 0, 0);

        assert_ok_eq!(value.parse_u16(), 0);
    }

    #[test]
    fn value_parse_u16_overflow() {
        let value = Value::new(b"65536", 0, 0);

        assert_err_eq!(
            value.parse_u16(),
            Error::new(error::Kind::ExpectedU16, 0, 0)
        );
    }

    #[test]
    fn value_parse_u16_invalid() {
        let value = Value::new(b"invalid", 0, 0);

        assert_err_eq!(
            value.parse_u16(),
            Error::new(error::Kind::ExpectedU16, 0, 0)
        );
    }

    #[test]
    fn value_parse_u16_whitespace() {
        let value = Value::new(b"  42 \n", 0, 0);

        assert_ok_eq!(value.parse_u16(), 42);
    }

    #[test]
    fn value_parse_u32() {
        let value = Value::new(b"42", 0, 0);

        assert_ok_eq!(value.parse_u32(), 42);
    }

    #[test]
    fn value_parse_u32_zero() {
        let value = Value::new(b"0", 0, 0);

        assert_ok_eq!(value.parse_u32(), 0);
    }

    #[test]
    fn value_parse_u32_overflow() {
        let value = Value::new(b"4294967296", 0, 0);

        assert_err_eq!(
            value.parse_u32(),
            Error::new(error::Kind::ExpectedU32, 0, 0)
        );
    }

    #[test]
    fn value_parse_u32_invalid() {
        let value = Value::new(b"invalid", 0, 0);

        assert_err_eq!(
            value.parse_u32(),
            Error::new(error::Kind::ExpectedU32, 0, 0)
        );
    }

    #[test]
    fn value_parse_u32_whitespace() {
        let value = Value::new(b"  42 \n", 0, 0);

        assert_ok_eq!(value.parse_u32(), 42);
    }

    #[test]
    fn value_parse_u64() {
        let value = Value::new(b"42", 0, 0);

        assert_ok_eq!(value.parse_u64(), 42);
    }

    #[test]
    fn value_parse_u64_zero() {
        let value = Value::new(b"0", 0, 0);

        assert_ok_eq!(value.parse_u64(), 0);
    }

    #[test]
    fn value_parse_u64_overflow() {
        let value = Value::new(b"18446744073709551616", 0, 0);

        assert_err_eq!(
            value.parse_u64(),
            Error::new(error::Kind::ExpectedU64, 0, 0)
        );
    }

    #[test]
    fn value_parse_u64_invalid() {
        let value = Value::new(b"invalid", 0, 0);

        assert_err_eq!(
            value.parse_u64(),
            Error::new(error::Kind::ExpectedU64, 0, 0)
        );
    }

    #[test]
    fn value_parse_u64_whitespace() {
        let value = Value::new(b"  42 \n", 0, 0);

        assert_ok_eq!(value.parse_u64(), 42);
    }

    #[test]
    fn value_parse_u128() {
        let value = Value::new(b"42", 0, 0);

        assert_ok_eq!(value.parse_u128(), 42);
    }

    #[test]
    fn value_parse_u128_zero() {
        let value = Value::new(b"0", 0, 0);

        assert_ok_eq!(value.parse_u128(), 0);
    }

    #[test]
    fn value_parse_u128_overflow() {
        let value = Value::new(b"340282366920938463463374607431768211456", 0, 0);

        assert_err_eq!(
            value.parse_u128(),
            Error::new(error::Kind::ExpectedU128, 0, 0)
        );
    }

    #[test]
    fn value_parse_u128_invalid() {
        let value = Value::new(b"invalid", 0, 0);

        assert_err_eq!(
            value.parse_u128(),
            Error::new(error::Kind::ExpectedU128, 0, 0)
        );
    }

    #[test]
    fn value_parse_u128_whitespace() {
        let value = Value::new(b"  42 \n", 0, 0);

        assert_ok_eq!(value.parse_u128(), 42);
    }

    #[test]
    fn value_parse_f32_positive() {
        let value = Value::new(b"42.9", 0, 0);

        assert_ok_eq!(value.parse_f32(), 42.9);
    }

    #[test]
    fn value_parse_f32_negative() {
        let value = Value::new(b"-42.9", 0, 0);

        assert_ok_eq!(value.parse_f32(), -42.9);
    }

    #[test]
    fn value_parse_f32_zero() {
        let value = Value::new(b"0", 0, 0);

        assert_ok_eq!(value.parse_f32(), 0.0);
    }

    #[test]
    fn value_parse_f32_positive_overflow() {
        let value = Value::new(b"3.40282347E+39", 0, 0);

        assert_ok_eq!(value.parse_f32(), f32::INFINITY,);
    }

    #[test]
    fn value_parse_f32_negative_overflow() {
        let value = Value::new(b"-3.40282347E+39", 0, 0);

        assert_ok_eq!(value.parse_f32(), f32::NEG_INFINITY,);
    }

    #[test]
    fn value_parse_f32_invalid() {
        let value = Value::new(b"invalid", 0, 0);

        assert_err_eq!(
            value.parse_f32(),
            Error::new(error::Kind::ExpectedF32, 0, 0)
        );
    }

    #[test]
    fn value_parse_f32_whitespace() {
        let value = Value::new(b"  42.9 \n", 0, 0);

        assert_ok_eq!(value.parse_f32(), 42.9);
    }

    #[test]
    fn value_parse_f64_positive() {
        let value = Value::new(b"42.9", 0, 0);

        assert_ok_eq!(value.parse_f64(), 42.9);
    }

    #[test]
    fn value_parse_f64_negative() {
        let value = Value::new(b"-42.9", 0, 0);

        assert_ok_eq!(value.parse_f64(), -42.9);
    }

    #[test]
    fn value_parse_f64_zero() {
        let value = Value::new(b"0", 0, 0);

        assert_ok_eq!(value.parse_f64(), 0.0);
    }

    #[test]
    fn value_parse_f64_positive_overflow() {
        let value = Value::new(b"1.7976931348623157E+309", 0, 0);

        assert_ok_eq!(value.parse_f64(), f64::INFINITY,);
    }

    #[test]
    fn value_parse_f64_negative_overflow() {
        let value = Value::new(b"-1.7976931348623157E+309", 0, 0);

        assert_ok_eq!(value.parse_f64(), f64::NEG_INFINITY,);
    }

    #[test]
    fn value_parse_f64_invalid() {
        let value = Value::new(b"invalid", 0, 0);

        assert_err_eq!(
            value.parse_f64(),
            Error::new(error::Kind::ExpectedF64, 0, 0)
        );
    }

    #[test]
    fn value_parse_f64_whitespace() {
        let value = Value::new(b"  42.9 \n", 0, 0);

        assert_ok_eq!(value.parse_f64(), 42.9);
    }
}
