use crate::de::{map, parse::Tags, r#enum, r#struct, tuple, Error, Result};
use serde::{de, de::Visitor};
use std::io::Read;

pub(in crate::de) struct Deserializer<'a, R> {
    tags: &'a mut Tags<R>,
}

impl<'a, R> Deserializer<'a, R> {
    pub(in crate::de) fn new(tags: &'a mut Tags<R>) -> Self {
        Self { tags }
    }
}

impl<'de, 'a, R> de::Deserializer<'de> for Deserializer<'a, R>
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
        visitor.visit_byte_buf(parsed)
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
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

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let result = visitor.visit_seq(tuple::Access::new(&mut values, len))?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        Ok(result)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let result = visitor.visit_seq(tuple::Access::new(&mut values, len))?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        Ok(result)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let result = visitor.visit_map(map::field::Access::new(&mut tag))?;
        tag.assert_exhausted()?;
        Ok(result)
    }

    fn deserialize_struct<V>(
        mut self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        value.parse_unit()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_map(r#struct::Access::new(&mut self.tags, fields))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let result = visitor.visit_enum(r#enum::Access::new(&mut values))?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        Ok(result)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        // Parsed string must be owned, since it removes escaping and comments.
        let parsed = value.parse_identifier()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_str(&parsed)
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
    use super::Deserializer;
    use crate::de::{error, parse::Tags, Error};
    use claim::{assert_err_eq, assert_ok_eq};
    use serde::{de, de::Visitor, Deserialize};
    use serde_bytes::ByteBuf;
    use serde_derive::Deserialize;
    use std::{collections::HashMap, fmt};

    #[test]
    fn bool_true() {
        let mut tags = Tags::new(b"#true;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(bool::deserialize(deserializer), true);
    }

    #[test]
    fn bool_false() {
        let mut tags = Tags::new(b"#false;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(bool::deserialize(deserializer), false);
    }

    #[test]
    fn bool_invalid() {
        let mut tags = Tags::new(b"#invalid;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            bool::deserialize(deserializer),
            Error::new(error::Kind::ExpectedBool, 0, 1)
        );
    }

    #[test]
    fn i8() {
        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(i8::deserialize(deserializer), 42);
    }

    #[test]
    fn i8_invalid() {
        let mut tags = Tags::new(b"#invalid;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i8::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI8, 0, 1)
        );
    }

    #[test]
    fn i8_too_many_values() {
        let mut tags = Tags::new(b"#42:100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i8::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    fn i8_unexpected_values() {
        let mut tags = Tags::new(b"#42;100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i8::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    fn i16() {
        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(i16::deserialize(deserializer), 42);
    }

    #[test]
    fn i16_invalid() {
        let mut tags = Tags::new(b"#invalid;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i16::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI16, 0, 1)
        );
    }

    #[test]
    fn i16_too_many_values() {
        let mut tags = Tags::new(b"#42:100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i16::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    fn i16_unexpected_values() {
        let mut tags = Tags::new(b"#42;100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i16::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    fn i32() {
        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(i32::deserialize(deserializer), 42);
    }

    #[test]
    fn i32_invalid() {
        let mut tags = Tags::new(b"#invalid;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i32::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI32, 0, 1)
        );
    }

    #[test]
    fn i32_too_many_values() {
        let mut tags = Tags::new(b"#42:100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    fn i32_unexpected_values() {
        let mut tags = Tags::new(b"#42;100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    fn i64() {
        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(i64::deserialize(deserializer), 42);
    }

    #[test]
    fn i64_invalid() {
        let mut tags = Tags::new(b"#invalid;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i64::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI64, 0, 1)
        );
    }

    #[test]
    fn i64_too_many_values() {
        let mut tags = Tags::new(b"#42:100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    fn i64_unexpected_values() {
        let mut tags = Tags::new(b"#42;100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128() {
        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(i128::deserialize(deserializer), 42);
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128_invalid() {
        let mut tags = Tags::new(b"#invalid;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i128::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI128, 0, 1)
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128_too_many_values() {
        let mut tags = Tags::new(b"#42:100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i128::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128_unexpected_values() {
        let mut tags = Tags::new(b"#42;100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i128::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    fn u8() {
        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(u8::deserialize(deserializer), 42);
    }

    #[test]
    fn u8_invalid() {
        let mut tags = Tags::new(b"#invalid;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u8::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU8, 0, 1)
        );
    }

    #[test]
    fn u8_too_many_values() {
        let mut tags = Tags::new(b"#42:100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u8::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    fn u8_unexpected_values() {
        let mut tags = Tags::new(b"#42;100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u8::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    fn u16() {
        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(u16::deserialize(deserializer), 42);
    }

    #[test]
    fn u16_invalid() {
        let mut tags = Tags::new(b"#invalid;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u16::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU16, 0, 1)
        );
    }

    #[test]
    fn u16_too_many_values() {
        let mut tags = Tags::new(b"#42:100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u16::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    fn u16_unexpected_values() {
        let mut tags = Tags::new(b"#42;100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u16::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    fn u32() {
        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(u32::deserialize(deserializer), 42);
    }

    #[test]
    fn u32_invalid() {
        let mut tags = Tags::new(b"#invalid;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u32::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU32, 0, 1)
        );
    }

    #[test]
    fn u32_too_many_values() {
        let mut tags = Tags::new(b"#42:100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    fn u32_unexpected_values() {
        let mut tags = Tags::new(b"#42;100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    fn u64() {
        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(u64::deserialize(deserializer), 42);
    }

    #[test]
    fn u64_invalid() {
        let mut tags = Tags::new(b"#invalid;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u64::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU64, 0, 1)
        );
    }

    #[test]
    fn u64_too_many_values() {
        let mut tags = Tags::new(b"#42:100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    fn u64_unexpected_values() {
        let mut tags = Tags::new(b"#42;100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128() {
        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(u128::deserialize(deserializer), 42);
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128_invalid() {
        let mut tags = Tags::new(b"#invalid;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u128::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU128, 0, 1)
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128_too_many_values() {
        let mut tags = Tags::new(b"#42:100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u128::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128_unexpected_values() {
        let mut tags = Tags::new(b"#42;100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u128::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    fn f32() {
        let mut tags = Tags::new(b"#42.9;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(f32::deserialize(deserializer), 42.9);
    }

    #[test]
    fn f32_invalid() {
        let mut tags = Tags::new(b"#invalid;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            f32::deserialize(deserializer),
            Error::new(error::Kind::ExpectedF32, 0, 1)
        );
    }

    #[test]
    fn f32_too_many_values() {
        let mut tags = Tags::new(b"#42.9:1.2;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            f32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 6)
        );
    }

    #[test]
    fn f32_unexpected_values() {
        let mut tags = Tags::new(b"#42.9;1.2;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            f32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 6)
        );
    }

    #[test]
    fn f64() {
        let mut tags = Tags::new(b"#42.9;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(f64::deserialize(deserializer), 42.9);
    }

    #[test]
    fn f64_invalid() {
        let mut tags = Tags::new(b"#invalid;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            f64::deserialize(deserializer),
            Error::new(error::Kind::ExpectedF64, 0, 1)
        );
    }

    #[test]
    fn f64_too_many_values() {
        let mut tags = Tags::new(b"#42.9:1.2;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            f64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 6)
        );
    }

    #[test]
    fn f64_unexpected_values() {
        let mut tags = Tags::new(b"#42.9;1.2;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            f64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 6)
        );
    }

    #[test]
    fn char() {
        let mut tags = Tags::new(b"#a;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(char::deserialize(deserializer), 'a');
    }

    #[test]
    fn char_invalid() {
        let mut tags = Tags::new(b"#invalid;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            char::deserialize(deserializer),
            Error::new(error::Kind::ExpectedChar, 0, 1)
        );
    }

    #[test]
    fn char_too_many_values() {
        let mut tags = Tags::new(b"#a:b;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            char::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 3)
        );
    }

    #[test]
    fn char_unexpected_values() {
        let mut tags = Tags::new(b"#a;b;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            char::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 3)
        );
    }

    #[test]
    fn string() {
        let mut tags = Tags::new(b"#foo;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(String::deserialize(deserializer), "foo".to_owned());
    }

    #[test]
    fn string_invalid() {
        let mut tags = Tags::new(b"#\xF0\x9Ffoo;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            String::deserialize(deserializer),
            Error::new(error::Kind::ExpectedString, 0, 1)
        );
    }

    #[test]
    fn string_too_many_values() {
        let mut tags = Tags::new(b"#foo:bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            String::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 5)
        );
    }

    #[test]
    fn string_unexpected_values() {
        let mut tags = Tags::new(b"#foo;bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            String::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 5)
        );
    }

    #[test]
    fn byte_buf() {
        let mut tags = Tags::new(b"#foo;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(ByteBuf::deserialize(deserializer), b"foo");
    }

    #[test]
    fn byte_buf_too_many_values() {
        let mut tags = Tags::new(b"#foo:bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            ByteBuf::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 5)
        );
    }

    #[test]
    fn byte_buf_unexpected_values() {
        let mut tags = Tags::new(b"#foo;bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            ByteBuf::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 5)
        );
    }

    #[test]
    fn unit() {
        let mut tags = Tags::new(b"#;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(<()>::deserialize(deserializer), ());
    }

    #[test]
    fn unit_invalid() {
        let mut tags = Tags::new(b"#invalid;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            <()>::deserialize(deserializer),
            Error::new(error::Kind::ExpectedUnit, 0, 1)
        );
    }

    #[test]
    fn unit_too_many_values() {
        let mut tags = Tags::new(b"#:;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            <()>::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 2)
        );
    }

    #[test]
    fn unit_unexpected_values() {
        let mut tags = Tags::new(b"#;;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            <()>::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 2)
        );
    }

    #[test]
    fn unit_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Unit;
        let mut tags = Tags::new(b"#;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(Unit::deserialize(deserializer), Unit);
    }

    #[test]
    fn unit_struct_invalid() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Unit;
        let mut tags = Tags::new(b"#invalid;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Unit::deserialize(deserializer),
            Error::new(error::Kind::ExpectedUnit, 0, 1)
        );
    }

    #[test]
    fn unit_struct_too_many_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Unit;
        let mut tags = Tags::new(b"#:;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Unit::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 2)
        );
    }

    #[test]
    fn unit_struct_unexpected_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Unit;
        let mut tags = Tags::new(b"#;;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Unit::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 2)
        );
    }

    #[test]
    fn newtype_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Newtype(u64);
        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(Newtype::deserialize(deserializer), Newtype(42));
    }

    #[test]
    fn tuple() {
        let mut tags = Tags::new(b"#42:foo::1.2;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(
            <(u64, String, (), f64)>::deserialize(deserializer),
            (42, "foo".to_owned(), (), 1.2)
        );
    }

    #[test]
    fn tuple_too_many_values() {
        let mut tags = Tags::new(b"#42:foo::1.2:bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            <(u64, String, (), f64)>::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 13)
        );
    }

    #[test]
    fn tuple_unexpected_values() {
        let mut tags = Tags::new(b"#42:foo::1.2;bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            <(u64, String, (), f64)>::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 13)
        );
    }

    #[test]
    fn tuple_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TupleStruct(u64, String, (), f64);
        let mut tags = Tags::new(b"#42:foo::1.2;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(
            TupleStruct::deserialize(deserializer),
            TupleStruct(42, "foo".to_owned(), (), 1.2)
        );
    }

    #[test]
    fn tuple_struct_too_many_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TupleStruct(u64, String, (), f64);
        let mut tags = Tags::new(b"#42:foo::1.2:bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            TupleStruct::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 13)
        );
    }

    #[test]
    fn tuple_struct_unexpected_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TupleStruct(u64, String, (), f64);
        let mut tags = Tags::new(b"#42:foo::1.2;bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            TupleStruct::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 13)
        );
    }

    #[test]
    fn map() {
        let mut tags = Tags::new(b"#foo:1;bar:2;baz:3;qux:4;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        let mut expected = HashMap::new();
        expected.insert("foo".to_owned(), 1);
        expected.insert("bar".to_owned(), 2);
        expected.insert("baz".to_owned(), 3);
        expected.insert("qux".to_owned(), 4);
        assert_ok_eq!(HashMap::<String, u64>::deserialize(deserializer), expected);
    }

    #[test]
    fn r#struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Struct {
            foo: String,
            bar: u64,
            baz: (),
            qux: f64,
        }
        let mut tags = Tags::new(b"#;\n#foo:test;\n#bar:42;\n#baz:;\n#qux:1.2;\n".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(
            Struct::deserialize(deserializer),
            Struct {
                foo: "test".to_owned(),
                bar: 42,
                baz: (),
                qux: 1.2
            }
        );
    }

    #[test]
    fn enum_unit_variant() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Unit {
            Variant,
        }
        let mut tags = Tags::new(b"#Variant;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(Unit::deserialize(deserializer), Unit::Variant);
    }

    #[test]
    fn enum_unit_variant_too_many_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Unit {
            Variant,
        }
        let mut tags = Tags::new(b"#Variant:42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Unit::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 9)
        );
    }

    #[test]
    fn enum_unit_variant_unexpected_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Unit {
            Variant,
        }
        let mut tags = Tags::new(b"#Variant;42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Unit::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 9)
        );
    }

    #[test]
    fn enum_newtype_variant() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Newtype {
            Variant(u64),
        }
        let mut tags = Tags::new(b"#Variant:42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(Newtype::deserialize(deserializer), Newtype::Variant(42));
    }

    #[test]
    fn enum_newtype_variant_too_many_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Newtype {
            Variant(u64),
        }
        let mut tags = Tags::new(b"#Variant:42:bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Newtype::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 12)
        );
    }

    #[test]
    fn enum_newtype_variant_unexpected_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Newtype {
            Variant(u64),
        }
        let mut tags = Tags::new(b"#Variant:42;bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Newtype::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 12)
        );
    }

    #[test]
    fn enum_tuple_variant() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Tuple {
            Variant(u64, String, (), f64),
        }
        let mut tags = Tags::new(b"#Variant:42:foo::1.2;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(
            Tuple::deserialize(deserializer),
            Tuple::Variant(42, "foo".to_owned(), (), 1.2)
        );
    }

    #[test]
    fn enum_tuple_variant_too_many_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Tuple {
            Variant(u64, String, (), f64),
        }
        let mut tags = Tags::new(b"#Variant:42:foo::1.2:bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Tuple::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 21)
        );
    }

    #[test]
    fn enum_tuple_variant_unexpected_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Tuple {
            Variant(u64, String, (), f64),
        }
        let mut tags = Tags::new(b"#Variant:42:foo::1.2;bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Tuple::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 21)
        );
    }

    #[derive(Debug, PartialEq)]
    struct Identifier(String);

    impl<'de> Deserialize<'de> for Identifier {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct IdentifierVisitor;

            impl<'de> Visitor<'de> for IdentifierVisitor {
                type Value = Identifier;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("identifier")
                }

                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Ok(Identifier(value.to_owned()))
                }
            }

            deserializer.deserialize_identifier(IdentifierVisitor)
        }
    }

    #[test]
    fn identifier() {
        let mut tags = Tags::new(b"#foo;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(
            Identifier::deserialize(deserializer),
            Identifier("foo".to_owned())
        );
    }

    #[test]
    fn identifier_whitespace() {
        let mut tags = Tags::new(b"#   foo  ;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(
            Identifier::deserialize(deserializer),
            Identifier("foo".to_owned())
        );
    }

    #[test]
    fn identifier_invalid() {
        let mut tags = Tags::new(b"#\xF0\x9Ffoo;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Identifier::deserialize(deserializer),
            Error::new(error::Kind::ExpectedIdentifier, 0, 1)
        );
    }

    #[test]
    fn identifier_too_many_values() {
        let mut tags = Tags::new(b"#foo:bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Identifier::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 5)
        );
    }

    #[test]
    fn identifier_unexpected_values() {
        let mut tags = Tags::new(b"#foo;bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Identifier::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 5)
        );
    }
}
