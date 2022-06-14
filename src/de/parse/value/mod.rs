mod clean;
mod trim;

use super::utf8_char_width::utf8_char_width;
use crate::de::{error, Error, Position, Result};
use arrayvec::ArrayVec;
use clean::Clean;
use either::Either;
use num_traits::{Float, PrimInt, Signed};
use std::str;
use trim::Trim;

fn parse_ident<I>(mut value_bytes: I, ident: &[u8]) -> bool
where
    I: Iterator<Item = u8>,
{
    for &ident_byte in ident {
        if let Some(value_byte) = value_bytes.next() {
            if ident_byte != value_byte {
                return false;
            }
        } else {
            return false;
        }
    }
    // If there are remaining characters, then the parsing fails.
    value_bytes.next().is_none()
}

fn parse_inf<I>(mut value_bytes: I) -> bool
where
    I: Iterator<Item = u8>,
{
    // Match `inf`.
    // Note that the `i` is already matched.
    for &byte in b"nf" {
        if let Some(value_byte) = value_bytes.next() {
            if byte != value_byte {
                return false;
            }
        } else {
            return false;
        }
    }
    match value_bytes.next() {
        None => true,
        Some(b'i') => {
            // Match "infinity".
            // Note that `infi` is already matched.
            parse_ident(value_bytes, b"nity")
        }
        _ => false,
    }
}

fn parse_positive_integer_inner<I, N>(value_bytes: I, mut result: Option<N>) -> Option<N>
where
    I: Iterator<Item = u8>,
    N: PrimInt,
{
    for byte in value_bytes {
        if result.is_none() {
            result = Some(N::zero());
        }
        result = Some({
            // SAFETY: After the above statement, `result` can't be `None` here.
            let prev_result = unsafe { result.unwrap_unchecked() };
            prev_result
                .checked_mul(
                    // SAFETY: 10 can be cast to any `N` safely.
                    &unsafe { N::from(10).unwrap_unchecked() },
                )?
                .checked_add(&match byte {
                    b'0'..=b'9' => {
                        // SAFETY: Casting this value to `N` will always succeed, because the maximum
                        // value it can be is 9.
                        unsafe { N::from::<u8>(byte - b'0').unwrap_unchecked() }
                    }
                    _ => {
                        return None;
                    }
                })?
        });
    }

    result
}

fn parse_unsigned_integer<I, N>(value_bytes: I) -> Option<N>
where
    I: Iterator<Item = u8>,
    N: PrimInt,
{
    parse_positive_integer_inner(value_bytes, None)
}

fn parse_signed_integer<I, N>(mut value_bytes: I) -> Option<N>
where
    I: Iterator<Item = u8>,
    N: PrimInt + Signed,
{
    match value_bytes.next()? {
        b'-' => {
            // Negative.
            let mut result = None;

            for byte in value_bytes {
                if result.is_none() {
                    result = Some(N::zero());
                }
                result = Some({
                    // SAFETY: After the above statement, `result` can't be `None` here.
                    let prev_result = unsafe { result.unwrap_unchecked() };
                    prev_result
                        .checked_mul(
                            // SAFETY: 10 can be cast to any `N` safely.
                            &unsafe { N::from(10).unwrap_unchecked() },
                        )?
                        .checked_sub(&match byte {
                            b'0'..=b'9' => {
                                // SAFETY: Casting this value to `N` will always succeed, because the maximum
                                // value it can be is 9.
                                unsafe { N::from::<u8>(byte - b'0').unwrap_unchecked() }
                            }
                            _ => {
                                return None;
                            }
                        })?
                });
            }

            result
        }
        byte @ b'0'..=b'9' => {
            // Positive.
            parse_positive_integer_inner(
                value_bytes,
                Some(
                    // SAFETY: Casting this value to `N` will always succeed, because the maximum value
                    // it can be is 9.
                    unsafe { N::from::<u8>(byte - b'0').unwrap_unchecked() },
                ),
            )
        }
        _ => None,
    }
}

fn parse_float<I, F>(mut value_bytes: I) -> Option<F>
where
    I: Iterator<Item = u8>,
    F: Float,
{
    enum State {
        Whole,
        Fraction,
        Exponent,
    }
    let mut state = State::Whole;

    // Parse sign.
    let mut first_byte = value_bytes.next()?;
    let negative = matches!(first_byte, b'-');
    if matches!(first_byte, b'+' | b'-') {
        first_byte = value_bytes.next()?;
    }

    let mut mantissa: u64 = match first_byte {
        b'0'..=b'9' => (first_byte - b'0').into(),
        b'.' => {
            state = State::Fraction;
            0
        }
        // Special case: infinite values.
        b'i' => {
            return if parse_inf(value_bytes) {
                if negative {
                    Some(F::neg_infinity())
                } else {
                    Some(F::infinity())
                }
            } else {
                None
            }
        }
        // Special case: not-a-number values.
        b'n' => {
            return if parse_ident(value_bytes, b"an") {
                Some(F::nan())
            } else {
                None
            }
        }
        _ => return None,
    };

    if matches!(state, State::Whole) {
        for byte in &mut value_bytes {
            match byte {
                b'0'..=b'9' => {
                    mantissa = match mantissa.checked_mul(10) {
                        Some(mantissa) => match mantissa.checked_add((byte - b'0').into()) {
                            Some(mantissa) => mantissa,
                            None => {
                                return Some(if negative {
                                    F::infinity()
                                } else {
                                    F::neg_infinity()
                                })
                            }
                        },
                        None => {
                            return Some(if negative {
                                F::infinity()
                            } else {
                                F::neg_infinity()
                            })
                        }
                    }
                }
                b'.' => {
                    state = State::Fraction;
                    break;
                }
                b'e' => {
                    state = State::Exponent;
                    break;
                }
                _ => return None,
            }
        }
    }

    let mut exponent: i32 = 0;
    if matches!(state, State::Fraction) {
        for byte in &mut value_bytes {
            match byte {
                b'0'..=b'9' => {
                    mantissa = match mantissa.checked_mul(10) {
                        Some(mantissa) => match mantissa.checked_add((byte - b'0').into()) {
                            Some(mantissa) => mantissa,
                            None => {
                                return Some(if negative {
                                    F::infinity()
                                } else {
                                    F::neg_infinity()
                                })
                            }
                        },
                        None => {
                            return Some(if negative {
                                F::infinity()
                            } else {
                                F::neg_infinity()
                            })
                        }
                    };
                    exponent = exponent.saturating_sub(1);
                }
                b'e' => {
                    state = State::Exponent;
                    break;
                }
                _ => return None,
            }
        }
    }

    if matches!(state, State::Exponent) {
        // Parse sign.
        let mut first_byte = value_bytes.next()?;
        let exponent_negative = matches!(first_byte, b'-');
        if matches!(first_byte, b'+' | b'-') {
            first_byte = value_bytes.next()?;
        }

        let exponent_number: i32 =
            parse_positive_integer_inner(value_bytes, Some((first_byte - b'0').into()))
                .unwrap_or(i32::MAX);
        if exponent_negative {
            exponent = exponent.saturating_sub(exponent_number);
        } else {
            exponent = exponent.saturating_add(exponent_number);
        }
    }

    // SAFETY: The conversion between u64 and F is guaranteed not to fail for any float type, since
    // it uses the `as` keyword under the hood.
    let mut value = unsafe { F::from(mantissa).unwrap_unchecked() };
    value = if exponent < 0 {
        // SAFETY: Converting f64 to F is infallible.
        value / unsafe { F::from(10f64.powi(-exponent)).unwrap_unchecked() }
    } else {
        // SAFETY: Converting f64 to F is infallible.
        value * unsafe { F::from(10f64.powi(exponent)).unwrap_unchecked() }
    };
    if negative {
        value = -value;
    }
    Some(value)
}

#[derive(Debug, PartialEq)]
pub(in crate::de) struct Value<'a> {
    bytes: &'a [u8],
    position: Position,
}

impl<'a> Value<'a> {
    pub(in crate::de) fn new(bytes: &'a [u8], position: Position) -> Self {
        Self { bytes, position }
    }

    pub(in crate::de) fn position(&self) -> Position {
        self.position
    }

    pub(in crate::de) fn parse_bool(&self) -> Result<bool> {
        let mut value = Trim::new(Clean::new(self.bytes));
        match value
            .next()
            .ok_or_else(|| Error::new(error::Kind::ExpectedBool, self.position))?
        {
            b't' => {
                if parse_ident(value, b"rue") {
                    Ok(true)
                } else {
                    Err(Error::new(error::Kind::ExpectedBool, self.position))
                }
            }
            b'f' => {
                if parse_ident(value, b"alse") {
                    Ok(false)
                } else {
                    Err(Error::new(error::Kind::ExpectedBool, self.position))
                }
            }
            _ => Err(Error::new(error::Kind::ExpectedBool, self.position)),
        }
    }

    pub(in crate::de) fn parse_i8(&self) -> Result<i8> {
        parse_signed_integer(Trim::new(Clean::new(self.bytes)))
            .ok_or_else(|| Error::new(error::Kind::ExpectedI8, self.position))
    }

    pub(in crate::de) fn parse_i16(&self) -> Result<i16> {
        parse_signed_integer(Trim::new(Clean::new(self.bytes)))
            .ok_or_else(|| Error::new(error::Kind::ExpectedI16, self.position))
    }

    pub(in crate::de) fn parse_i32(&self) -> Result<i32> {
        parse_signed_integer(Trim::new(Clean::new(self.bytes)))
            .ok_or_else(|| Error::new(error::Kind::ExpectedI32, self.position))
    }

    pub(in crate::de) fn parse_i64(&self) -> Result<i64> {
        parse_signed_integer(Trim::new(Clean::new(self.bytes)))
            .ok_or_else(|| Error::new(error::Kind::ExpectedI64, self.position))
    }

    #[cfg(has_i128)]
    pub(in crate::de) fn parse_i128(&self) -> Result<i128> {
        parse_signed_integer(Trim::new(Clean::new(self.bytes)))
            .ok_or_else(|| Error::new(error::Kind::ExpectedI128, self.position))
    }

    pub(in crate::de) fn parse_u8(&self) -> Result<u8> {
        parse_unsigned_integer(Trim::new(Clean::new(self.bytes)))
            .ok_or_else(|| Error::new(error::Kind::ExpectedU8, self.position))
    }

    pub(in crate::de) fn parse_u16(&self) -> Result<u16> {
        parse_unsigned_integer(Trim::new(Clean::new(self.bytes)))
            .ok_or_else(|| Error::new(error::Kind::ExpectedU16, self.position))
    }

    pub(in crate::de) fn parse_u32(&self) -> Result<u32> {
        parse_unsigned_integer(Trim::new(Clean::new(self.bytes)))
            .ok_or_else(|| Error::new(error::Kind::ExpectedU32, self.position))
    }

    pub(in crate::de) fn parse_u64(&self) -> Result<u64> {
        parse_unsigned_integer(Trim::new(Clean::new(self.bytes)))
            .ok_or_else(|| Error::new(error::Kind::ExpectedU64, self.position))
    }

    #[cfg(has_i128)]
    pub(in crate::de) fn parse_u128(&self) -> Result<u128> {
        parse_unsigned_integer(Trim::new(Clean::new(self.bytes)))
            .ok_or_else(|| Error::new(error::Kind::ExpectedU128, self.position))
    }

    pub(in crate::de) fn parse_f32(&self) -> Result<f32> {
        parse_float(Trim::new(Clean::new(self.bytes)).map(|b| b.to_ascii_lowercase()))
            .ok_or_else(|| Error::new(error::Kind::ExpectedF32, self.position))
    }

    pub(in crate::de) fn parse_f64(&self) -> Result<f64> {
        parse_float(Trim::new(Clean::new(self.bytes)).map(|b| b.to_ascii_lowercase()))
            .ok_or_else(|| Error::new(error::Kind::ExpectedF64, self.position))
    }

    pub(in crate::de) fn parse_char(&self) -> Result<char> {
        let cleaned = Clean::new(self.bytes);
        // Try to trim.
        let mut value = Either::Left(Trim::new(cleaned.clone()));
        let first_byte = if let Some(byte) = value.next() {
            byte
        } else {
            value = Either::Right(cleaned);
            if let Some(byte) = value.next() {
                byte
            } else {
                return Err(Error::new(error::Kind::ExpectedChar, self.position));
            }
        };

        let width = utf8_char_width(first_byte);
        if width == 0 {
            Err(Error::new(error::Kind::ExpectedChar, self.position))
        } else if width == 1 {
            if value.next().is_none() {
                Ok(first_byte as char)
            } else {
                Err(Error::new(error::Kind::ExpectedChar, self.position))
            }
        } else {
            let mut buffer = ArrayVec::<_, 4>::new();
            // SAFETY: The buffer is empty, and therefore can hold at least this single element.
            unsafe { buffer.push_unchecked(first_byte) };
            for _ in 0..3 {
                if let Some(byte) = value.next() {
                    // SAFETY: The buffer is guaranteed to contain less than 4 elements within this loop,
                    // because we enter the loop with only one element and only add a maximum of 3 during
                    // the loop.
                    unsafe { buffer.push_unchecked(byte) };
                } else {
                    break;
                }
            }
            if value.next().is_none() && buffer.len() == width {
                Ok(str::from_utf8(buffer.as_slice())
                    .map_err(|_| Error::new(error::Kind::ExpectedChar, self.position))
                    .map(|s|
                        // SAFETY: Since `from_utf8()` returned a string, we can guarantee it has exactly
                        // one value, since the width indicated by the first byte was exactly the length of
                        // the input and the input was nonempty.
                        Ok(unsafe { s.chars().next().unwrap_unchecked() }))?)?
            } else {
                Err(Error::new(error::Kind::ExpectedChar, self.position))
            }
        }
    }

    pub(in crate::de) fn parse_string(&self) -> Result<String> {
        String::from_utf8(Clean::new(self.bytes).collect::<Vec<u8>>())
            .map_err(|_| Error::new(error::Kind::ExpectedString, self.position))
    }

    pub(in crate::de) fn parse_byte_buf(&self) -> Vec<u8> {
        Clean::new(self.bytes).collect()
    }

    pub(in crate::de) fn parse_unit(&self) -> Result<()> {
        // A unit must contain only whitespace and comments.
        if Clean::new(self.bytes).all(|b| b.is_ascii_whitespace()) {
            Ok(())
        } else {
            Err(Error::new(error::Kind::ExpectedUnit, self.position))
        }
    }

    pub(in crate::de) fn parse_identifier(&self) -> Result<String> {
        String::from_utf8(Trim::new(Clean::new(self.bytes)).collect::<Vec<u8>>())
            .map_err(|_| Error::new(error::Kind::ExpectedIdentifier, self.position))
    }
}

#[cfg(test)]
mod tests {
    use super::Value;
    use crate::de::{error, Error, Position};
    use claim::{assert_err_eq, assert_ok, assert_ok_eq};

    #[test]
    fn get_position() {
        let value = Value::new(b"", Position::new(1, 2));

        assert_eq!(value.position(), Position::new(1, 2));
    }

    #[test]
    fn parse_bool_true() {
        let value = Value::new(b"true", Position::new(0, 0));

        assert_ok_eq!(value.parse_bool(), true);
    }

    #[test]
    fn parse_bool_false() {
        let value = Value::new(b"false", Position::new(0, 0));

        assert_ok_eq!(value.parse_bool(), false);
    }

    #[test]
    fn parse_bool_invalid() {
        let value = Value::new(b"not a bool", Position::new(0, 0));

        assert_err_eq!(
            value.parse_bool(),
            Error::new(error::Kind::ExpectedBool, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_i8_positive() {
        let value = Value::new(b"42", Position::new(0, 0));

        assert_ok_eq!(value.parse_i8(), 42);
    }

    #[test]
    fn parse_i8_negative() {
        let value = Value::new(b"-42", Position::new(0, 0));

        assert_ok_eq!(value.parse_i8(), -42);
    }

    #[test]
    fn parse_i8_zero() {
        let value = Value::new(b"0", Position::new(0, 0));

        assert_ok_eq!(value.parse_i8(), 0);
    }

    #[test]
    fn parse_i8_positive_overflow() {
        let value = Value::new(b"128", Position::new(0, 0));

        assert_err_eq!(
            value.parse_i8(),
            Error::new(error::Kind::ExpectedI8, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_i8_negative_overflow() {
        let value = Value::new(b"-129", Position::new(0, 0));

        assert_err_eq!(
            value.parse_i8(),
            Error::new(error::Kind::ExpectedI8, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_i8_invalid() {
        let value = Value::new(b"invalid", Position::new(0, 0));

        assert_err_eq!(
            value.parse_i8(),
            Error::new(error::Kind::ExpectedI8, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_i8_whitespace() {
        let value = Value::new(b"  42 \n", Position::new(0, 0));

        assert_ok_eq!(value.parse_i8(), 42);
    }

    #[test]
    fn parse_i16_positive() {
        let value = Value::new(b"42", Position::new(0, 0));

        assert_ok_eq!(value.parse_i16(), 42);
    }

    #[test]
    fn parse_i16_negative() {
        let value = Value::new(b"-42", Position::new(0, 0));

        assert_ok_eq!(value.parse_i16(), -42);
    }

    #[test]
    fn parse_i16_zero() {
        let value = Value::new(b"0", Position::new(0, 0));

        assert_ok_eq!(value.parse_i16(), 0);
    }

    #[test]
    fn parse_i16_positive_overflow() {
        let value = Value::new(b"32768", Position::new(0, 0));

        assert_err_eq!(
            value.parse_i16(),
            Error::new(error::Kind::ExpectedI16, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_i16_negative_overflow() {
        let value = Value::new(b"-32769", Position::new(0, 0));

        assert_err_eq!(
            value.parse_i16(),
            Error::new(error::Kind::ExpectedI16, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_i16_invalid() {
        let value = Value::new(b"invalid", Position::new(0, 0));

        assert_err_eq!(
            value.parse_i16(),
            Error::new(error::Kind::ExpectedI16, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_i16_whitespace() {
        let value = Value::new(b"  42 \n", Position::new(0, 0));

        assert_ok_eq!(value.parse_i16(), 42);
    }

    #[test]
    fn parse_i32_positive() {
        let value = Value::new(b"42", Position::new(0, 0));

        assert_ok_eq!(value.parse_i32(), 42);
    }

    #[test]
    fn parse_i32_negative() {
        let value = Value::new(b"-42", Position::new(0, 0));

        assert_ok_eq!(value.parse_i32(), -42);
    }

    #[test]
    fn parse_i32_zero() {
        let value = Value::new(b"0", Position::new(0, 0));

        assert_ok_eq!(value.parse_i32(), 0);
    }

    #[test]
    fn parse_i32_positive_overflow() {
        let value = Value::new(b"2147483648", Position::new(0, 0));

        assert_err_eq!(
            value.parse_i32(),
            Error::new(error::Kind::ExpectedI32, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_i32_negative_overflow() {
        let value = Value::new(b"-2147483649", Position::new(0, 0));

        assert_err_eq!(
            value.parse_i32(),
            Error::new(error::Kind::ExpectedI32, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_i32_invalid() {
        let value = Value::new(b"invalid", Position::new(0, 0));

        assert_err_eq!(
            value.parse_i32(),
            Error::new(error::Kind::ExpectedI32, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_i32_whitespace() {
        let value = Value::new(b"  42 \n", Position::new(0, 0));

        assert_ok_eq!(value.parse_i32(), 42);
    }

    #[test]
    fn parse_i64_positive() {
        let value = Value::new(b"42", Position::new(0, 0));

        assert_ok_eq!(value.parse_i64(), 42);
    }

    #[test]
    fn parse_i64_negative() {
        let value = Value::new(b"-42", Position::new(0, 0));

        assert_ok_eq!(value.parse_i64(), -42);
    }

    #[test]
    fn parse_i64_zero() {
        let value = Value::new(b"0", Position::new(0, 0));

        assert_ok_eq!(value.parse_i64(), 0);
    }

    #[test]
    fn parse_i64_positive_overflow() {
        let value = Value::new(b"9223372036854775808", Position::new(0, 0));

        assert_err_eq!(
            value.parse_i64(),
            Error::new(error::Kind::ExpectedI64, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_i64_negative_overflow() {
        let value = Value::new(b"-9223372036854775809", Position::new(0, 0));

        assert_err_eq!(
            value.parse_i64(),
            Error::new(error::Kind::ExpectedI64, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_i64_invalid() {
        let value = Value::new(b"invalid", Position::new(0, 0));

        assert_err_eq!(
            value.parse_i64(),
            Error::new(error::Kind::ExpectedI64, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_i64_whitespace() {
        let value = Value::new(b"  42 \n", Position::new(0, 0));

        assert_ok_eq!(value.parse_i64(), 42);
    }

    #[test]
    fn parse_i128_positive() {
        let value = Value::new(b"42", Position::new(0, 0));

        assert_ok_eq!(value.parse_i128(), 42);
    }

    #[test]
    fn parse_i128_negative() {
        let value = Value::new(b"-42", Position::new(0, 0));

        assert_ok_eq!(value.parse_i128(), -42);
    }

    #[test]
    fn parse_i128_zero() {
        let value = Value::new(b"0", Position::new(0, 0));

        assert_ok_eq!(value.parse_i128(), 0);
    }

    #[test]
    fn parse_i128_positive_overflow() {
        let value = Value::new(
            b"170141183460469231731687303715884105728",
            Position::new(0, 0),
        );

        assert_err_eq!(
            value.parse_i128(),
            Error::new(error::Kind::ExpectedI128, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_i128_negative_overflow() {
        let value = Value::new(
            b"-170141183460469231731687303715884105729",
            Position::new(0, 0),
        );

        assert_err_eq!(
            value.parse_i128(),
            Error::new(error::Kind::ExpectedI128, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_i128_invalid() {
        let value = Value::new(b"invalid", Position::new(0, 0));

        assert_err_eq!(
            value.parse_i128(),
            Error::new(error::Kind::ExpectedI128, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_i128_whitespace() {
        let value = Value::new(b"  42 \n", Position::new(0, 0));

        assert_ok_eq!(value.parse_i128(), 42);
    }

    #[test]
    fn parse_u8() {
        let value = Value::new(b"42", Position::new(0, 0));

        assert_ok_eq!(value.parse_u8(), 42);
    }

    #[test]
    fn parse_u8_zero() {
        let value = Value::new(b"0", Position::new(0, 0));

        assert_ok_eq!(value.parse_u8(), 0);
    }

    #[test]
    fn parse_u8_overflow() {
        let value = Value::new(b"256", Position::new(0, 0));

        assert_err_eq!(
            value.parse_u8(),
            Error::new(error::Kind::ExpectedU8, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_u8_invalid() {
        let value = Value::new(b"invalid", Position::new(0, 0));

        assert_err_eq!(
            value.parse_u8(),
            Error::new(error::Kind::ExpectedU8, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_u8_whitespace() {
        let value = Value::new(b"  42 \n", Position::new(0, 0));

        assert_ok_eq!(value.parse_u8(), 42);
    }

    #[test]
    fn parse_u16() {
        let value = Value::new(b"42", Position::new(0, 0));

        assert_ok_eq!(value.parse_u16(), 42);
    }

    #[test]
    fn parse_u16_zero() {
        let value = Value::new(b"0", Position::new(0, 0));

        assert_ok_eq!(value.parse_u16(), 0);
    }

    #[test]
    fn parse_u16_overflow() {
        let value = Value::new(b"65536", Position::new(0, 0));

        assert_err_eq!(
            value.parse_u16(),
            Error::new(error::Kind::ExpectedU16, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_u16_invalid() {
        let value = Value::new(b"invalid", Position::new(0, 0));

        assert_err_eq!(
            value.parse_u16(),
            Error::new(error::Kind::ExpectedU16, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_u16_whitespace() {
        let value = Value::new(b"  42 \n", Position::new(0, 0));

        assert_ok_eq!(value.parse_u16(), 42);
    }

    #[test]
    fn parse_u32() {
        let value = Value::new(b"42", Position::new(0, 0));

        assert_ok_eq!(value.parse_u32(), 42);
    }

    #[test]
    fn parse_u32_zero() {
        let value = Value::new(b"0", Position::new(0, 0));

        assert_ok_eq!(value.parse_u32(), 0);
    }

    #[test]
    fn parse_u32_overflow() {
        let value = Value::new(b"4294967296", Position::new(0, 0));

        assert_err_eq!(
            value.parse_u32(),
            Error::new(error::Kind::ExpectedU32, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_u32_invalid() {
        let value = Value::new(b"invalid", Position::new(0, 0));

        assert_err_eq!(
            value.parse_u32(),
            Error::new(error::Kind::ExpectedU32, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_u32_whitespace() {
        let value = Value::new(b"  42 \n", Position::new(0, 0));

        assert_ok_eq!(value.parse_u32(), 42);
    }

    #[test]
    fn parse_u64() {
        let value = Value::new(b"42", Position::new(0, 0));

        assert_ok_eq!(value.parse_u64(), 42);
    }

    #[test]
    fn parse_u64_zero() {
        let value = Value::new(b"0", Position::new(0, 0));

        assert_ok_eq!(value.parse_u64(), 0);
    }

    #[test]
    fn parse_u64_overflow() {
        let value = Value::new(b"18446744073709551616", Position::new(0, 0));

        assert_err_eq!(
            value.parse_u64(),
            Error::new(error::Kind::ExpectedU64, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_u64_invalid() {
        let value = Value::new(b"invalid", Position::new(0, 0));

        assert_err_eq!(
            value.parse_u64(),
            Error::new(error::Kind::ExpectedU64, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_u64_whitespace() {
        let value = Value::new(b"  42 \n", Position::new(0, 0));

        assert_ok_eq!(value.parse_u64(), 42);
    }

    #[test]
    fn parse_u128() {
        let value = Value::new(b"42", Position::new(0, 0));

        assert_ok_eq!(value.parse_u128(), 42);
    }

    #[test]
    fn parse_u128_zero() {
        let value = Value::new(b"0", Position::new(0, 0));

        assert_ok_eq!(value.parse_u128(), 0);
    }

    #[test]
    fn parse_u128_overflow() {
        let value = Value::new(
            b"340282366920938463463374607431768211456",
            Position::new(0, 0),
        );

        assert_err_eq!(
            value.parse_u128(),
            Error::new(error::Kind::ExpectedU128, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_u128_invalid() {
        let value = Value::new(b"invalid", Position::new(0, 0));

        assert_err_eq!(
            value.parse_u128(),
            Error::new(error::Kind::ExpectedU128, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_u128_whitespace() {
        let value = Value::new(b"  42 \n", Position::new(0, 0));

        assert_ok_eq!(value.parse_u128(), 42);
    }

    #[test]
    fn parse_f32_positive() {
        let value = Value::new(b"42.9", Position::new(0, 0));

        assert_ok_eq!(value.parse_f32(), 42.9);
    }

    #[test]
    fn parse_f32_negative() {
        let value = Value::new(b"-42.9", Position::new(0, 0));

        assert_ok_eq!(value.parse_f32(), -42.9);
    }

    #[test]
    fn parse_f32_zero() {
        let value = Value::new(b"0", Position::new(0, 0));

        assert_ok_eq!(value.parse_f32(), 0.0);
    }

    #[test]
    fn parse_f32_positive_overflow() {
        let value = Value::new(b"3.40282347E+39", Position::new(0, 0));

        assert_ok_eq!(value.parse_f32(), f32::INFINITY,);
    }

    #[test]
    fn parse_f32_negative_overflow() {
        let value = Value::new(b"-3.40282347E+39", Position::new(0, 0));

        assert_ok_eq!(value.parse_f32(), f32::NEG_INFINITY,);
    }

    #[test]
    fn parse_f32_invalid() {
        let value = Value::new(b"invalid", Position::new(0, 0));

        assert_err_eq!(
            value.parse_f32(),
            Error::new(error::Kind::ExpectedF32, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_f32_nan() {
        let value = Value::new(b"NaN", Position::new(0, 0));

        let result = assert_ok!(value.parse_f32());
        assert!(result.is_nan());
    }

    #[test]
    fn parse_f32_negative_nan() {
        let value = Value::new(b"-NaN", Position::new(0, 0));

        let result = assert_ok!(value.parse_f32());
        assert!(result.is_nan());
    }

    #[test]
    fn parse_f32_infinity() {
        let value = Value::new(b"INF", Position::new(0, 0));

        assert_ok_eq!(value.parse_f32(), f32::INFINITY);
    }

    #[test]
    fn parse_f32_negative_infinity() {
        let value = Value::new(b"-infinity", Position::new(0, 0));

        assert_ok_eq!(value.parse_f32(), f32::NEG_INFINITY);
    }

    #[test]
    fn parse_f32_whitespace() {
        let value = Value::new(b"  42.9 \n", Position::new(0, 0));

        assert_ok_eq!(value.parse_f32(), 42.9);
    }

    #[test]
    fn parse_f64_positive() {
        let value = Value::new(b"42.9", Position::new(0, 0));

        assert_ok_eq!(value.parse_f64(), 42.9);
    }

    #[test]
    fn parse_f64_negative() {
        let value = Value::new(b"-42.9", Position::new(0, 0));

        assert_ok_eq!(value.parse_f64(), -42.9);
    }

    #[test]
    fn parse_f64_zero() {
        let value = Value::new(b"0", Position::new(0, 0));

        assert_ok_eq!(value.parse_f64(), 0.0);
    }

    #[test]
    fn parse_f64_positive_overflow() {
        let value = Value::new(b"1.7976931348623157E+309", Position::new(0, 0));

        assert_ok_eq!(value.parse_f64(), f64::INFINITY,);
    }

    #[test]
    fn parse_f64_negative_overflow() {
        let value = Value::new(b"-1.7976931348623157E+309", Position::new(0, 0));

        assert_ok_eq!(value.parse_f64(), f64::NEG_INFINITY,);
    }

    #[test]
    fn parse_f64_invalid() {
        let value = Value::new(b"invalid", Position::new(0, 0));

        assert_err_eq!(
            value.parse_f64(),
            Error::new(error::Kind::ExpectedF64, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_f64_whitespace() {
        let value = Value::new(b"  42.9 \n", Position::new(0, 0));

        assert_ok_eq!(value.parse_f64(), 42.9);
    }

    #[test]
    fn parse_f64_nan() {
        let value = Value::new(b"NaN", Position::new(0, 0));

        let result = assert_ok!(value.parse_f64());
        assert!(result.is_nan());
    }

    #[test]
    fn parse_f64_negative_nan() {
        let value = Value::new(b"-NaN", Position::new(0, 0));

        let result = assert_ok!(value.parse_f64());
        assert!(result.is_nan());
    }

    #[test]
    fn parse_f64_infinity() {
        let value = Value::new(b"INF", Position::new(0, 0));

        assert_ok_eq!(value.parse_f64(), f64::INFINITY);
    }

    #[test]
    fn parse_f64_negative_infinity() {
        let value = Value::new(b"-infinity", Position::new(0, 0));

        assert_ok_eq!(value.parse_f64(), f64::NEG_INFINITY);
    }

    #[test]
    fn parse_char() {
        let value = Value::new(b"a", Position::new(0, 0));

        assert_ok_eq!(value.parse_char(), 'a');
    }

    #[test]
    fn parse_char_longer() {
        let value = Value::new(b"\xF0\x9F\x92\xA3", Position::new(0, 0));

        assert_ok_eq!(value.parse_char(), '💣');
    }

    #[test]
    fn parse_char_surrounded_by_whitespace() {
        let value = Value::new(b"\n \ta  \t", Position::new(0, 0));

        assert_ok_eq!(value.parse_char(), 'a');
    }

    #[test]
    fn parse_char_whitespace() {
        let value = Value::new(b"\t", Position::new(0, 0));

        assert_ok_eq!(value.parse_char(), '\t');
    }

    #[test]
    fn parse_char_multiple_whitespaces() {
        let value = Value::new(b"\t\n", Position::new(0, 0));

        // Can't deduce which whitespace character is meant.
        assert_err_eq!(
            value.parse_char(),
            Error::new(error::Kind::ExpectedChar, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_char_incomplete_char() {
        let value = Value::new(b"\xF0", Position::new(0, 0));

        assert_err_eq!(
            value.parse_char(),
            Error::new(error::Kind::ExpectedChar, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_char_invalid_first_byte() {
        let value = Value::new(b"\x92", Position::new(0, 0));

        assert_err_eq!(
            value.parse_char(),
            Error::new(error::Kind::ExpectedChar, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_char_multiple_chars() {
        let value = Value::new(b"abc", Position::new(0, 0));

        assert_err_eq!(
            value.parse_char(),
            Error::new(error::Kind::ExpectedChar, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_string() {
        let value = Value::new(b"foo", Position::new(0, 0));

        assert_ok_eq!(value.parse_string(), "foo".to_owned(),);
    }

    #[test]
    fn parse_string_escaped() {
        let value = Value::new(b"\\#foo\\\\bar", Position::new(0, 0));

        assert_ok_eq!(value.parse_string(), "#foo\\bar".to_owned(),);
    }

    #[test]
    fn parse_string_comment() {
        let value = Value::new(b"foo\n// comment\nbar", Position::new(0, 0));

        assert_ok_eq!(value.parse_string(), "foo\n\nbar".to_owned(),);
    }

    #[test]
    fn parse_string_fails() {
        let value = Value::new(b"\xF0\x9Ffoo", Position::new(0, 0));

        assert_err_eq!(
            value.parse_string(),
            Error::new(error::Kind::ExpectedString, Position::new(0, 0)),
        );
    }

    #[test]
    fn parse_byte_buf() {
        let value = Value::new(b"foo", Position::new(0, 0));

        assert_eq!(value.parse_byte_buf(), b"foo",);
    }

    #[test]
    fn parse_byte_buf_escaped() {
        let value = Value::new(b"\\#foo\\\\bar", Position::new(0, 0));

        assert_eq!(value.parse_byte_buf(), b"#foo\\bar",);
    }

    #[test]
    fn parse_byte_buf_comment() {
        let value = Value::new(b"foo\n// comment\nbar", Position::new(0, 0));

        assert_eq!(value.parse_byte_buf(), b"foo\n\nbar",);
    }

    #[test]
    fn parse_byte_buf_non_ascii() {
        let value = Value::new(b"\xF0\x9Ffoo", Position::new(0, 0));

        assert_eq!(value.parse_byte_buf(), b"\xF0\x9Ffoo",);
    }

    #[test]
    fn parse_unit() {
        let value = Value::new(b"", Position::new(0, 0));

        assert_ok!(value.parse_unit());
    }

    #[test]
    fn parse_unit_fails() {
        let value = Value::new(b"foo", Position::new(0, 0));

        assert_err_eq!(
            value.parse_unit(),
            Error::new(error::Kind::ExpectedUnit, Position::new(0, 0))
        );
    }

    #[test]
    fn parse_unit_ignores_whitespace() {
        let value = Value::new(b"  \n\t ", Position::new(0, 0));

        assert_ok!(value.parse_unit());
    }

    #[test]
    fn parse_unit_ignores_comments() {
        let value = Value::new(b"//comment\n", Position::new(0, 0));

        assert_ok!(value.parse_unit());
    }

    #[test]
    fn parse_identifier() {
        let value = Value::new(b"foo", Position::new(0, 0));

        assert_ok_eq!(value.parse_identifier(), "foo".to_owned());
    }

    #[test]
    fn parse_identifier_escaped() {
        let value = Value::new(b"foo\\:", Position::new(0, 0));

        assert_ok_eq!(value.parse_identifier(), "foo:".to_owned());
    }

    #[test]
    fn parse_identifier_comment() {
        let value = Value::new(b"foo//comment", Position::new(0, 0));

        assert_ok_eq!(value.parse_identifier(), "foo".to_owned());
    }

    #[test]
    fn parse_identifier_trim_whitespace() {
        let value = Value::new(b" \t foo\n   \n", Position::new(0, 0));

        assert_ok_eq!(value.parse_identifier(), "foo".to_owned());
    }

    #[test]
    fn parse_identifier_whitespace_and_comment() {
        let value = Value::new(b"foo   //comment", Position::new(0, 0));

        assert_ok_eq!(value.parse_identifier(), "foo".to_owned());
    }

    #[test]
    fn parse_identifier_invalid() {
        let value = Value::new(b"\xF0\x9Ffoo", Position::new(0, 0));

        assert_err_eq!(
            value.parse_identifier(),
            Error::new(error::Kind::ExpectedIdentifier, Position::new(0, 0))
        );
    }
}
