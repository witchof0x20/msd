mod error;
mod parse;

pub use error::{Error, Result};

use serde::{de, de::Visitor};
use std::io::Read;

pub struct Deserializer<R> {
    tags: parse::Tags<R>,
}

impl<R> Deserializer<R> {
    pub fn new(reader: R) -> Self
    where
        R: Read,
    {
        Self {
            tags: parse::Tags::new(reader),
        }
    }
}

impl<'de, 'a, R> de::Deserializer<'de> for &'a mut Deserializer<R>
where
    R: Read,
{
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let parsed = value.parse_bool()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        self.tags.assert_exhausted()?;
        visitor.visit_bool(parsed)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let parsed = value.parse_i8()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        self.tags.assert_exhausted()?;
        visitor.visit_i8(parsed)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let parsed = value.parse_i16()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        self.tags.assert_exhausted()?;
        visitor.visit_i16(parsed)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let parsed = value.parse_i32()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        self.tags.assert_exhausted()?;
        visitor.visit_i32(parsed)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let parsed = value.parse_i64()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        self.tags.assert_exhausted()?;
        visitor.visit_i64(parsed)
    }

    #[cfg(has_i128)]
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let parsed = value.parse_i128()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        self.tags.assert_exhausted()?;
        visitor.visit_i128(parsed)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let parsed = value.parse_u8()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        self.tags.assert_exhausted()?;
        visitor.visit_u8(parsed)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let parsed = value.parse_u16()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        self.tags.assert_exhausted()?;
        visitor.visit_u16(parsed)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let parsed = value.parse_u32()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        self.tags.assert_exhausted()?;
        visitor.visit_u32(parsed)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let parsed = value.parse_u64()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        self.tags.assert_exhausted()?;
        visitor.visit_u64(parsed)
    }

    #[cfg(has_i128)]
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let parsed = value.parse_u128()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        self.tags.assert_exhausted()?;
        visitor.visit_u128(parsed)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let parsed = value.parse_f32()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        self.tags.assert_exhausted()?;
        visitor.visit_f32(parsed)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let parsed = value.parse_f64()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        self.tags.assert_exhausted()?;
        visitor.visit_f64(parsed)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let parsed = value.parse_char()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        self.tags.assert_exhausted()?;
        visitor.visit_char(parsed)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        // Parsed string must be owned, since it removes escaping and comments.
        let parsed = value.parse_string()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        self.tags.assert_exhausted()?;
        visitor.visit_str(&parsed)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let parsed = value.parse_string()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        self.tags.assert_exhausted()?;
        visitor.visit_string(parsed)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        // Parsed bytes must be owned, since it removes escaping and comments.
        let parsed = value.parse_byte_buf();
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        self.tags.assert_exhausted()?;
        visitor.visit_bytes(&parsed)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let parsed = value.parse_byte_buf();
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        self.tags.assert_exhausted()?;
        visitor.visit_byte_buf(parsed)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.tags.has_next()? {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        value.parse_unit()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        self.tags.assert_exhausted()?;
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        value.parse_unit()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        self.tags.assert_exhausted()?;
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{error, Deserializer, Error};
    use claim::{assert_err_eq, assert_ok_eq};
    use serde::Deserialize;
    use serde_bytes::ByteBuf;
    use serde_derive::Deserialize;

    #[test]
    fn bool_true() {
        let mut deserializer = Deserializer::new(b"#true;".as_slice());

        assert_ok_eq!(bool::deserialize(&mut deserializer), true);
    }

    #[test]
    fn bool_false() {
        let mut deserializer = Deserializer::new(b"#false;".as_slice());

        assert_ok_eq!(bool::deserialize(&mut deserializer), false);
    }

    #[test]
    fn bool_invalid() {
        let mut deserializer = Deserializer::new(b"#invalid;".as_slice());

        assert_err_eq!(
            bool::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedBool, 0, 1)
        );
    }

    #[test]
    fn i8_positive() {
        let mut deserializer = Deserializer::new(b"#42;".as_slice());

        assert_ok_eq!(i8::deserialize(&mut deserializer), 42);
    }

    #[test]
    fn i8_negative() {
        let mut deserializer = Deserializer::new(b"#-42;".as_slice());

        assert_ok_eq!(i8::deserialize(&mut deserializer), -42);
    }

    #[test]
    fn i8_zero() {
        let mut deserializer = Deserializer::new(b"#0;".as_slice());

        assert_ok_eq!(i8::deserialize(&mut deserializer), 0);
    }

    #[test]
    fn i8_positive_overflow() {
        let mut deserializer = Deserializer::new(b"#128;".as_slice());

        assert_err_eq!(
            i8::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedI8, 0, 1)
        );
    }

    #[test]
    fn i8_negative_overflow() {
        let mut deserializer = Deserializer::new(b"#-129;".as_slice());

        assert_err_eq!(
            i8::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedI8, 0, 1)
        );
    }

    #[test]
    fn i8_invalid() {
        let mut deserializer = Deserializer::new(b"#invalid;".as_slice());

        assert_err_eq!(
            i8::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedI8, 0, 1)
        );
    }

    #[test]
    fn i16_positive() {
        let mut deserializer = Deserializer::new(b"#42;".as_slice());

        assert_ok_eq!(i16::deserialize(&mut deserializer), 42);
    }

    #[test]
    fn i16_negative() {
        let mut deserializer = Deserializer::new(b"#-42;".as_slice());

        assert_ok_eq!(i16::deserialize(&mut deserializer), -42);
    }

    #[test]
    fn i16_zero() {
        let mut deserializer = Deserializer::new(b"#0;".as_slice());

        assert_ok_eq!(i16::deserialize(&mut deserializer), 0);
    }

    #[test]
    fn i16_positive_overflow() {
        let mut deserializer = Deserializer::new(b"#32768;".as_slice());

        assert_err_eq!(
            i16::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedI16, 0, 1)
        );
    }

    #[test]
    fn i16_negative_overflow() {
        let mut deserializer = Deserializer::new(b"#-32769;".as_slice());

        assert_err_eq!(
            i16::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedI16, 0, 1)
        );
    }

    #[test]
    fn i16_invalid() {
        let mut deserializer = Deserializer::new(b"#invalid;".as_slice());

        assert_err_eq!(
            i16::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedI16, 0, 1)
        );
    }

    #[test]
    fn i32_positive() {
        let mut deserializer = Deserializer::new(b"#42;".as_slice());

        assert_ok_eq!(i32::deserialize(&mut deserializer), 42);
    }

    #[test]
    fn i32_negative() {
        let mut deserializer = Deserializer::new(b"#-42;".as_slice());

        assert_ok_eq!(i32::deserialize(&mut deserializer), -42);
    }

    #[test]
    fn i32_zero() {
        let mut deserializer = Deserializer::new(b"#0;".as_slice());

        assert_ok_eq!(i32::deserialize(&mut deserializer), 0);
    }

    #[test]
    fn i32_positive_overflow() {
        let mut deserializer = Deserializer::new(b"#2147483648;".as_slice());

        assert_err_eq!(
            i8::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedI8, 0, 1)
        );
    }

    #[test]
    fn i32_negative_overflow() {
        let mut deserializer = Deserializer::new(b"#-2147483649;".as_slice());

        assert_err_eq!(
            i8::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedI8, 0, 1)
        );
    }

    #[test]
    fn i32_invalid() {
        let mut deserializer = Deserializer::new(b"#invalid;".as_slice());

        assert_err_eq!(
            i8::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedI8, 0, 1)
        );
    }

    #[test]
    fn i64_positive() {
        let mut deserializer = Deserializer::new(b"#42;".as_slice());

        assert_ok_eq!(i64::deserialize(&mut deserializer), 42);
    }

    #[test]
    fn i64_negative() {
        let mut deserializer = Deserializer::new(b"#-42;".as_slice());

        assert_ok_eq!(i64::deserialize(&mut deserializer), -42);
    }

    #[test]
    fn i64_zero() {
        let mut deserializer = Deserializer::new(b"#0;".as_slice());

        assert_ok_eq!(i64::deserialize(&mut deserializer), 0);
    }

    #[test]
    fn i64_positive_overflow() {
        let mut deserializer = Deserializer::new(b"#9223372036854775808;".as_slice());

        assert_err_eq!(
            i64::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedI64, 0, 1)
        );
    }

    #[test]
    fn i64_negative_overflow() {
        let mut deserializer = Deserializer::new(b"#-9223372036854775809;".as_slice());

        assert_err_eq!(
            i64::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedI64, 0, 1)
        );
    }

    #[test]
    fn i64_invalid() {
        let mut deserializer = Deserializer::new(b"#invalid;".as_slice());

        assert_err_eq!(
            i64::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedI64, 0, 1)
        );
    }

    #[test]
    fn i128_positive() {
        let mut deserializer = Deserializer::new(b"#42;".as_slice());

        assert_ok_eq!(i128::deserialize(&mut deserializer), 42);
    }

    #[test]
    fn i128_negative() {
        let mut deserializer = Deserializer::new(b"#-42;".as_slice());

        assert_ok_eq!(i128::deserialize(&mut deserializer), -42);
    }

    #[test]
    fn i128_zero() {
        let mut deserializer = Deserializer::new(b"#0;".as_slice());

        assert_ok_eq!(i128::deserialize(&mut deserializer), 0);
    }

    #[test]
    fn i128_positive_overflow() {
        let mut deserializer =
            Deserializer::new(b"#170141183460469231731687303715884105728;".as_slice());

        assert_err_eq!(
            i128::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedI128, 0, 1)
        );
    }

    #[test]
    fn i128_negative_overflow() {
        let mut deserializer =
            Deserializer::new(b"#-170141183460469231731687303715884105729;".as_slice());

        assert_err_eq!(
            i128::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedI128, 0, 1)
        );
    }

    #[test]
    fn i128_invalid() {
        let mut deserializer = Deserializer::new(b"#invalid;".as_slice());

        assert_err_eq!(
            i128::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedI128, 0, 1)
        );
    }

    #[test]
    fn u8() {
        let mut deserializer = Deserializer::new(b"#42;".as_slice());

        assert_ok_eq!(u8::deserialize(&mut deserializer), 42);
    }

    #[test]
    fn u8_zero() {
        let mut deserializer = Deserializer::new(b"#0;".as_slice());

        assert_ok_eq!(u8::deserialize(&mut deserializer), 0);
    }

    #[test]
    fn u8_overflow() {
        let mut deserializer = Deserializer::new(b"#256;".as_slice());

        assert_err_eq!(
            u8::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedU8, 0, 1)
        );
    }

    #[test]
    fn u8_invalid() {
        let mut deserializer = Deserializer::new(b"#invalid;".as_slice());

        assert_err_eq!(
            u8::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedU8, 0, 1)
        );
    }

    #[test]
    fn u16() {
        let mut deserializer = Deserializer::new(b"#42;".as_slice());

        assert_ok_eq!(u16::deserialize(&mut deserializer), 42);
    }

    #[test]
    fn u16_zero() {
        let mut deserializer = Deserializer::new(b"#0;".as_slice());

        assert_ok_eq!(u16::deserialize(&mut deserializer), 0);
    }

    #[test]
    fn u16_overflow() {
        let mut deserializer = Deserializer::new(b"#65536;".as_slice());

        assert_err_eq!(
            u16::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedU16, 0, 1)
        );
    }

    #[test]
    fn u16_invalid() {
        let mut deserializer = Deserializer::new(b"#invalid;".as_slice());

        assert_err_eq!(
            u16::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedU16, 0, 1)
        );
    }

    #[test]
    fn u32() {
        let mut deserializer = Deserializer::new(b"#42;".as_slice());

        assert_ok_eq!(u32::deserialize(&mut deserializer), 42);
    }

    #[test]
    fn u32_zero() {
        let mut deserializer = Deserializer::new(b"#0;".as_slice());

        assert_ok_eq!(u32::deserialize(&mut deserializer), 0);
    }

    #[test]
    fn u32_overflow() {
        let mut deserializer = Deserializer::new(b"#4294967296;".as_slice());

        assert_err_eq!(
            u32::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedU32, 0, 1)
        );
    }

    #[test]
    fn u32_invalid() {
        let mut deserializer = Deserializer::new(b"#invalid;".as_slice());

        assert_err_eq!(
            u32::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedU32, 0, 1)
        );
    }

    #[test]
    fn u64() {
        let mut deserializer = Deserializer::new(b"#42;".as_slice());

        assert_ok_eq!(u64::deserialize(&mut deserializer), 42);
    }

    #[test]
    fn u64_zero() {
        let mut deserializer = Deserializer::new(b"#0;".as_slice());

        assert_ok_eq!(u64::deserialize(&mut deserializer), 0);
    }

    #[test]
    fn u64_overflow() {
        let mut deserializer = Deserializer::new(b"#18446744073709551616;".as_slice());

        assert_err_eq!(
            u64::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedU64, 0, 1)
        );
    }

    #[test]
    fn u64_invalid() {
        let mut deserializer = Deserializer::new(b"#invalid;".as_slice());

        assert_err_eq!(
            u64::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedU64, 0, 1)
        );
    }

    #[test]
    fn u128() {
        let mut deserializer = Deserializer::new(b"#42;".as_slice());

        assert_ok_eq!(u128::deserialize(&mut deserializer), 42);
    }

    #[test]
    fn u128_zero() {
        let mut deserializer = Deserializer::new(b"#0;".as_slice());

        assert_ok_eq!(u128::deserialize(&mut deserializer), 0);
    }

    #[test]
    fn u128_overflow() {
        let mut deserializer =
            Deserializer::new(b"#340282366920938463463374607431768211456;".as_slice());

        assert_err_eq!(
            u128::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedU128, 0, 1)
        );
    }

    #[test]
    fn u128_invalid() {
        let mut deserializer = Deserializer::new(b"#invalid;".as_slice());

        assert_err_eq!(
            u128::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedU128, 0, 1)
        );
    }

    #[test]
    fn f32_positive() {
        let mut deserializer = Deserializer::new(b"#42.9;".as_slice());

        assert_ok_eq!(f32::deserialize(&mut deserializer), 42.9);
    }

    #[test]
    fn f32_negative() {
        let mut deserializer = Deserializer::new(b"#-42.9;".as_slice());

        assert_ok_eq!(f32::deserialize(&mut deserializer), -42.9);
    }

    #[test]
    fn f32_zero() {
        let mut deserializer = Deserializer::new(b"#0;".as_slice());

        assert_ok_eq!(f32::deserialize(&mut deserializer), 0.0);
    }

    #[test]
    fn f32_positive_overflow() {
        let mut deserializer = Deserializer::new(b"#3.40282347E+39;".as_slice());

        assert_ok_eq!(f32::deserialize(&mut deserializer), f32::INFINITY,);
    }

    #[test]
    fn f32_negative_overflow() {
        let mut deserializer = Deserializer::new(b"#-3.40282347E+39;".as_slice());

        assert_ok_eq!(f32::deserialize(&mut deserializer), f32::NEG_INFINITY);
    }

    #[test]
    fn f32_invalid() {
        let mut deserializer = Deserializer::new(b"#invalid;".as_slice());

        assert_err_eq!(
            f32::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedF32, 0, 1)
        );
    }

    #[test]
    fn f64_positive() {
        let mut deserializer = Deserializer::new(b"#42.9;".as_slice());

        assert_ok_eq!(f64::deserialize(&mut deserializer), 42.9);
    }

    #[test]
    fn f64_negative() {
        let mut deserializer = Deserializer::new(b"#-42.9;".as_slice());

        assert_ok_eq!(f64::deserialize(&mut deserializer), -42.9);
    }

    #[test]
    fn f64_zero() {
        let mut deserializer = Deserializer::new(b"#0;".as_slice());

        assert_ok_eq!(f64::deserialize(&mut deserializer), 0.0);
    }

    #[test]
    fn f64_positive_overflow() {
        let mut deserializer = Deserializer::new(b"#1.7976931348623157E+309;".as_slice());

        assert_ok_eq!(f64::deserialize(&mut deserializer), f64::INFINITY,);
    }

    #[test]
    fn f64_negative_overflow() {
        let mut deserializer = Deserializer::new(b"#-1.7976931348623157E+309;".as_slice());

        assert_ok_eq!(f64::deserialize(&mut deserializer), f64::NEG_INFINITY);
    }

    #[test]
    fn f64_invalid() {
        let mut deserializer = Deserializer::new(b"#invalid;".as_slice());

        assert_err_eq!(
            f64::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedF64, 0, 1)
        );
    }

    #[test]
    fn char() {
        let mut deserializer = Deserializer::new(b"#a;".as_slice());

        assert_ok_eq!(char::deserialize(&mut deserializer), 'a');
    }

    #[test]
    fn char_empty() {
        let mut deserializer = Deserializer::new(b"".as_slice());

        assert_err_eq!(
            char::deserialize(&mut deserializer),
            Error::new(error::Kind::EndOfFile, 0, 0)
        );
    }

    #[test]
    fn char_unexpected_tag() {
        let mut deserializer = Deserializer::new(b"#a;#b;".as_slice());

        assert_err_eq!(
            char::deserialize(&mut deserializer),
            Error::new(error::Kind::UnexpectedTag, 0, 3)
        );
    }

    #[test]
    fn char_unexpected_values() {
        let mut deserializer = Deserializer::new(b"#a;b;".as_slice());

        assert_err_eq!(
            char::deserialize(&mut deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 3)
        );
    }

    #[test]
    fn char_unexpected_value() {
        let mut deserializer = Deserializer::new(b"#a:b;\n".as_slice());

        assert_err_eq!(
            char::deserialize(&mut deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 3)
        );
    }

    #[test]
    fn char_invalid() {
        let mut deserializer = Deserializer::new(b"#abc;\n".as_slice());

        assert_err_eq!(
            char::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedChar, 0, 1),
        );
    }

    #[test]
    fn string() {
        let mut deserializer = Deserializer::new(b"#foo;".as_slice());

        assert_ok_eq!(String::deserialize(&mut deserializer), "foo".to_string(),);
    }

    #[test]
    fn string_unexpected_tag() {
        let mut deserializer = Deserializer::new(b"#foo;#bar;".as_slice());

        assert_err_eq!(
            String::deserialize(&mut deserializer),
            Error::new(error::Kind::UnexpectedTag, 0, 5)
        );
    }

    #[test]
    fn string_unexpected_values() {
        let mut deserializer = Deserializer::new(b"#foo;bar;".as_slice());

        assert_err_eq!(
            String::deserialize(&mut deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 5)
        );
    }

    #[test]
    fn string_unexpected_value() {
        let mut deserializer = Deserializer::new(b"#foo:bar;\n".as_slice());

        assert_err_eq!(
            String::deserialize(&mut deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 5)
        );
    }

    #[test]
    fn string_invalid() {
        let mut deserializer = Deserializer::new(b"#\xF0\x9Ffoo;\n".as_slice());

        assert_err_eq!(
            String::deserialize(&mut deserializer),
            Error::new(error::Kind::ExpectedString, 0, 1),
        );
    }

    #[test]
    fn byte_buf() {
        let mut deserializer = Deserializer::new(b"#foo;".as_slice());

        assert_ok_eq!(ByteBuf::deserialize(&mut deserializer), b"foo",);
    }

    #[test]
    fn byte_buf_unexpected_tag() {
        let mut deserializer = Deserializer::new(b"#foo;#bar;".as_slice());

        assert_err_eq!(
            ByteBuf::deserialize(&mut deserializer),
            Error::new(error::Kind::UnexpectedTag, 0, 5)
        );
    }

    #[test]
    fn byte_buf_unexpected_values() {
        let mut deserializer = Deserializer::new(b"#foo;bar;".as_slice());

        assert_err_eq!(
            ByteBuf::deserialize(&mut deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 5)
        );
    }

    #[test]
    fn byte_buf_unexpected_value() {
        let mut deserializer = Deserializer::new(b"#foo:bar;\n".as_slice());

        assert_err_eq!(
            ByteBuf::deserialize(&mut deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 5)
        );
    }

    #[test]
    fn byte_buf_non_ascii() {
        let mut deserializer = Deserializer::new(b"#\xF0\x9Ffoo;\n".as_slice());

        assert_ok_eq!(ByteBuf::deserialize(&mut deserializer), b"\xF0\x9Ffoo",);
    }

    #[test]
    fn none() {
        let mut deserializer = Deserializer::new(b"\n\n".as_slice());

        assert_ok_eq!(Option::<()>::deserialize(&mut deserializer), None);
    }

    #[test]
    fn some_integer() {
        let mut deserializer = Deserializer::new(b"#42;\n".as_slice());

        assert_ok_eq!(Option::<u64>::deserialize(&mut deserializer), Some(42));
    }

    #[test]
    fn unit() {
        let mut deserializer = Deserializer::new(b"#;".as_slice());

        assert_ok_eq!(<()>::deserialize(&mut deserializer), ());
    }

    #[test]
    fn unit_invalid() {
        let mut deserializer = Deserializer::new(b"#invalid;".as_slice());

        assert_err_eq!(<()>::deserialize(&mut deserializer), Error::new(error::Kind::ExpectedUnit, 0, 1));
    }

    #[test]
    fn unit_unexpected_tag() {
        let mut deserializer = Deserializer::new(b"#;\n#;".as_slice());

        assert_err_eq!(<()>::deserialize(&mut deserializer), Error::new(error::Kind::UnexpectedTag, 1, 0));
    }

    #[test]
    fn unit_unexpected_values() {
        let mut deserializer = Deserializer::new(b"#;\n;".as_slice());

        assert_err_eq!(<()>::deserialize(&mut deserializer), Error::new(error::Kind::UnexpectedValues, 1, 0));
    }

    #[test]
    fn unit_unexpected_value() {
        let mut deserializer = Deserializer::new(b"#:;\n".as_slice());

        assert_err_eq!(<()>::deserialize(&mut deserializer), Error::new(error::Kind::UnexpectedValue, 0, 2));
    }

    #[test]
    fn unit_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Unit;
        let mut deserializer = Deserializer::new(b"#;".as_slice());

        assert_ok_eq!(Unit::deserialize(&mut deserializer), Unit);
    }

    #[test]
    fn unit_struct_invalid() {
        #[derive(Debug, Deserialize)]
        struct Unit;
        let mut deserializer = Deserializer::new(b"#invalid;".as_slice());

        assert_err_eq!(Unit::deserialize(&mut deserializer), Error::new(error::Kind::ExpectedUnit, 0, 1));
    }

    #[test]
    fn unit_struct_unexpected_tag() {
        #[derive(Debug, Deserialize)]
        struct Unit;
        let mut deserializer = Deserializer::new(b"#;\n#;".as_slice());

        assert_err_eq!(Unit::deserialize(&mut deserializer), Error::new(error::Kind::UnexpectedTag, 1, 0));
    }

    #[test]
    fn unit_struct_unexpected_values() {
        #[derive(Debug, Deserialize)]
        struct Unit;
        let mut deserializer = Deserializer::new(b"#;\n;".as_slice());

        assert_err_eq!(Unit::deserialize(&mut deserializer), Error::new(error::Kind::UnexpectedValues, 1, 0));
    }

    #[test]
    fn unit_struct_unexpected_value() {
        #[derive(Debug, Deserialize)]
        struct Unit;
        let mut deserializer = Deserializer::new(b"#:;\n".as_slice());

        assert_err_eq!(Unit::deserialize(&mut deserializer), Error::new(error::Kind::UnexpectedValue, 0, 2));
    }

    #[test]
    fn newtype_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Newtype(u64);
        let mut deserializer = Deserializer::new(b"#42;\n".as_slice());

        assert_ok_eq!(Newtype::deserialize(&mut deserializer), Newtype(42));
    }
}
