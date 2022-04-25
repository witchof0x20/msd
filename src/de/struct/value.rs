use crate::de::{
    map,
    parse::{Tag, Values},
    r#enum, tuple, Error, Result,
};
use serde::{de, de::Visitor};

pub(in super::super) struct Deserializer<'a> {
    tag: Tag<'a>,
    values: Values<'a>,
}

impl<'a> Deserializer<'a> {
    pub(in super::super) fn new(tag: Tag<'a>, values: Values<'a>) -> Self {
        Self { tag, values }
    }
}

impl<'a, 'de> de::Deserializer<'de> for Deserializer<'a> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        let parsed = value.parse_bool()?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        visitor.visit_bool(parsed)
    }

    fn deserialize_i8<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        let parsed = value.parse_i8()?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        visitor.visit_i8(parsed)
    }

    fn deserialize_i16<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        let parsed = value.parse_i16()?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        visitor.visit_i16(parsed)
    }

    fn deserialize_i32<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        let parsed = value.parse_i32()?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        visitor.visit_i32(parsed)
    }

    fn deserialize_i64<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        let parsed = value.parse_i64()?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        visitor.visit_i64(parsed)
    }

    #[cfg(has_i128)]
    fn deserialize_i128<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        let parsed = value.parse_i128()?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        visitor.visit_i128(parsed)
    }

    fn deserialize_u8<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        let parsed = value.parse_u8()?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        visitor.visit_u8(parsed)
    }

    fn deserialize_u16<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        let parsed = value.parse_u16()?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        visitor.visit_u16(parsed)
    }

    fn deserialize_u32<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        let parsed = value.parse_u32()?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        visitor.visit_u32(parsed)
    }

    fn deserialize_u64<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        let parsed = value.parse_u64()?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        visitor.visit_u64(parsed)
    }

    #[cfg(has_i128)]
    fn deserialize_u128<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        let parsed = value.parse_u128()?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        visitor.visit_u128(parsed)
    }

    fn deserialize_f32<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        let parsed = value.parse_f32()?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        visitor.visit_f32(parsed)
    }

    fn deserialize_f64<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        let parsed = value.parse_f64()?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        visitor.visit_f64(parsed)
    }

    fn deserialize_char<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        let parsed = value.parse_char()?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        visitor.visit_char(parsed)
    }

    fn deserialize_str<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        let parsed = value.parse_string()?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        visitor.visit_str(&parsed)
    }

    fn deserialize_string<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        let parsed = value.parse_string()?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        visitor.visit_string(parsed)
    }

    fn deserialize_bytes<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        let parsed = value.parse_byte_buf();
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        visitor.visit_bytes(&parsed)
    }

    fn deserialize_byte_buf<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        let parsed = value.parse_byte_buf();
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        visitor.visit_byte_buf(parsed)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // If a field is present, it must be a `Some` value.
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        value.parse_unit()?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(mut self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        value.parse_unit()?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
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

    fn deserialize_tuple<V>(mut self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let result = visitor.visit_seq(tuple::Access::new(&mut self.values, len))?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        Ok(result)
    }

    fn deserialize_tuple_struct<V>(
        mut self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let result = visitor.visit_seq(tuple::Access::new(&mut self.values, len))?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        Ok(result)
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // SAFETY: `self.values` references the same buffer that `self.tag` references.
        unsafe { self.tag.revisit(self.values) };
        let result = visitor.visit_map(map::field::Access::new(&mut self.tag))?;
        self.tag.assert_exhausted()?;
        Ok(result)
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
        mut self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let result = visitor.visit_enum(r#enum::Access::new(&mut self.values))?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
        Ok(result)
    }

    fn deserialize_identifier<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        let parsed = value.parse_identifier()?;
        self.values.assert_exhausted()?;
        self.tag.assert_exhausted()?;
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
    use crate::de::{error, parse::Tag, Error};
    use claim::{assert_err_eq, assert_ok, assert_ok_eq};
    use serde::{de, de::Visitor, Deserialize};
    use serde_bytes::ByteBuf;
    use serde_derive::Deserialize;
    use std::{collections::HashMap, fmt};

    #[test]
    fn bool_true() {
        let mut tag = Tag::new(b"true;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(bool::deserialize(deserializer), true);
    }

    #[test]
    fn bool_false() {
        let mut tag = Tag::new(b"false;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(bool::deserialize(deserializer), false);
    }

    #[test]
    fn bool_invalid() {
        let mut tag = Tag::new(b"invalid;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            bool::deserialize(deserializer),
            Error::new(error::Kind::ExpectedBool, 0, 1)
        );
    }

    #[test]
    fn bool_too_many_values() {
        let mut tag = Tag::new(b"true:true;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            bool::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 6)
        );
    }

    #[test]
    fn bool_unexpected_values() {
        let mut tag = Tag::new(b"true;true;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            bool::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 6)
        );
    }

    #[test]
    fn i8() {
        let mut tag = Tag::new(b"42;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(i8::deserialize(deserializer), 42);
    }

    #[test]
    fn i8_invalid() {
        let mut tag = Tag::new(b"invalid;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            i8::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI8, 0, 1)
        );
    }

    #[test]
    fn i8_too_many_values() {
        let mut tag = Tag::new(b"42:100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            i8::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    fn i8_unexpected_values() {
        let mut tag = Tag::new(b"42;100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            i8::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    fn i16() {
        let mut tag = Tag::new(b"42;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(i16::deserialize(deserializer), 42);
    }

    #[test]
    fn i16_invalid() {
        let mut tag = Tag::new(b"invalid;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            i16::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI16, 0, 1)
        );
    }

    #[test]
    fn i16_too_many_values() {
        let mut tag = Tag::new(b"42:100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            i16::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    fn i16_unexpected_values() {
        let mut tag = Tag::new(b"42;100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            i16::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    fn i32() {
        let mut tag = Tag::new(b"42;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(i32::deserialize(deserializer), 42);
    }

    #[test]
    fn i32_invalid() {
        let mut tag = Tag::new(b"invalid;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            i32::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI32, 0, 1)
        );
    }

    #[test]
    fn i32_too_many_values() {
        let mut tag = Tag::new(b"42:100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            i32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    fn i32_unexpected_values() {
        let mut tag = Tag::new(b"42;100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            i32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    fn i64() {
        let mut tag = Tag::new(b"42;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(i64::deserialize(deserializer), 42);
    }

    #[test]
    fn i64_invalid() {
        let mut tag = Tag::new(b"invalid;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            i64::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI64, 0, 1)
        );
    }

    #[test]
    fn i64_too_many_values() {
        let mut tag = Tag::new(b"42:100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            i64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    fn i64_unexpected_values() {
        let mut tag = Tag::new(b"42;100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            i64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128() {
        let mut tag = Tag::new(b"42;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(i128::deserialize(deserializer), 42);
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128_invalid() {
        let mut tag = Tag::new(b"invalid;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            i128::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI128, 0, 1)
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128_too_many_values() {
        let mut tag = Tag::new(b"42:100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            i128::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128_unexpected_values() {
        let mut tag = Tag::new(b"42;100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            i128::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    fn u8() {
        let mut tag = Tag::new(b"42;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(u8::deserialize(deserializer), 42);
    }

    #[test]
    fn u8_invalid() {
        let mut tag = Tag::new(b"invalid;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            u8::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU8, 0, 1)
        );
    }

    #[test]
    fn u8_too_many_values() {
        let mut tag = Tag::new(b"42:100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            u8::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    fn u8_unexpected_values() {
        let mut tag = Tag::new(b"42;100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            u8::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    fn u16() {
        let mut tag = Tag::new(b"42;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(u16::deserialize(deserializer), 42);
    }

    #[test]
    fn u16_invalid() {
        let mut tag = Tag::new(b"invalid;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            u16::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU16, 0, 1)
        );
    }

    #[test]
    fn u16_too_many_values() {
        let mut tag = Tag::new(b"42:100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            u16::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    fn u16_unexpected_values() {
        let mut tag = Tag::new(b"42;100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            u16::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    fn u32() {
        let mut tag = Tag::new(b"42;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(u32::deserialize(deserializer), 42);
    }

    #[test]
    fn u32_invalid() {
        let mut tag = Tag::new(b"invalid;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            u32::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU32, 0, 1)
        );
    }

    #[test]
    fn u32_too_many_values() {
        let mut tag = Tag::new(b"42:100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            u32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    fn u32_unexpected_values() {
        let mut tag = Tag::new(b"42;100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            u32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    fn u64() {
        let mut tag = Tag::new(b"42;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(u64::deserialize(deserializer), 42);
    }

    #[test]
    fn u64_invalid() {
        let mut tag = Tag::new(b"invalid;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            u64::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU64, 0, 1)
        );
    }

    #[test]
    fn u64_too_many_values() {
        let mut tag = Tag::new(b"42:100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            u64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    fn u64_unexpected_values() {
        let mut tag = Tag::new(b"42;100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            u64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128() {
        let mut tag = Tag::new(b"42;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(u128::deserialize(deserializer), 42);
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128_invalid() {
        let mut tag = Tag::new(b"invalid;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            u128::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU128, 0, 1)
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128_too_many_values() {
        let mut tag = Tag::new(b"42:100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            u128::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128_unexpected_values() {
        let mut tag = Tag::new(b"42;100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            u128::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    fn f32() {
        let mut tag = Tag::new(b"42.9;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(f32::deserialize(deserializer), 42.9);
    }

    #[test]
    fn f32_invalid() {
        let mut tag = Tag::new(b"invalid;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            f32::deserialize(deserializer),
            Error::new(error::Kind::ExpectedF32, 0, 1)
        );
    }

    #[test]
    fn f32_too_many_values() {
        let mut tag = Tag::new(b"42:100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            f32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    fn f32_unexpected_values() {
        let mut tag = Tag::new(b"42;100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            f32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    fn f64() {
        let mut tag = Tag::new(b"42.9;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(f64::deserialize(deserializer), 42.9);
    }

    #[test]
    fn f64_invalid() {
        let mut tag = Tag::new(b"invalid;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            f64::deserialize(deserializer),
            Error::new(error::Kind::ExpectedF64, 0, 1)
        );
    }

    #[test]
    fn f64_too_many_values() {
        let mut tag = Tag::new(b"42:100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            f64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 4)
        );
    }

    #[test]
    fn f64_unexpected_values() {
        let mut tag = Tag::new(b"42;100;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            f64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 4)
        );
    }

    #[test]
    fn char() {
        let mut tag = Tag::new(b"a;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(char::deserialize(deserializer), 'a');
    }

    #[test]
    fn char_invalid() {
        let mut tag = Tag::new(b"invalid;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            char::deserialize(deserializer),
            Error::new(error::Kind::ExpectedChar, 0, 1)
        );
    }

    #[test]
    fn char_too_many_values() {
        let mut tag = Tag::new(b"a:b;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            char::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 3)
        );
    }

    #[test]
    fn char_unexpected_values() {
        let mut tag = Tag::new(b"a;b;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            char::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 3)
        );
    }

    #[test]
    fn string() {
        let mut tag = Tag::new(b"foo;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(String::deserialize(deserializer), "foo");
    }

    #[test]
    fn string_invalid() {
        let mut tag = Tag::new(b"\xF0\x9Ffoo;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            String::deserialize(deserializer),
            Error::new(error::Kind::ExpectedString, 0, 1)
        );
    }

    #[test]
    fn string_too_many_values() {
        let mut tag = Tag::new(b"foo:bar;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            String::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 5)
        );
    }

    #[test]
    fn string_unexpected_values() {
        let mut tag = Tag::new(b"foo;bar;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            String::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 5)
        );
    }

    #[test]
    fn bytes() {
        let mut tag = Tag::new(b"foo;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(ByteBuf::deserialize(deserializer), b"foo");
    }

    #[test]
    fn bytes_too_many_values() {
        let mut tag = Tag::new(b"foo:bar;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            ByteBuf::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 5)
        );
    }

    #[test]
    fn bytes_unexpected_values() {
        let mut tag = Tag::new(b"foo;bar;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            ByteBuf::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 5)
        );
    }

    #[test]
    fn some() {
        let mut tag = Tag::new(b"42;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(Option::<u64>::deserialize(deserializer), Some(42));
    }

    #[test]
    fn unit() {
        let mut tag = Tag::new(b";", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(<()>::deserialize(deserializer), ());
    }

    #[test]
    fn unit_invalid() {
        let mut tag = Tag::new(b"invalid;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            <()>::deserialize(deserializer),
            Error::new(error::Kind::ExpectedUnit, 0, 1)
        );
    }

    #[test]
    fn unit_too_many_values() {
        let mut tag = Tag::new(b":;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            <()>::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 2)
        );
    }

    #[test]
    fn unit_unexpected_values() {
        let mut tag = Tag::new(b";;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            <()>::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 2)
        );
    }

    #[test]
    fn unit_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Unit;
        let mut tag = Tag::new(b";", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(Unit::deserialize(deserializer), Unit);
    }

    #[test]
    fn unit_struct_invalid() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Unit;
        let mut tag = Tag::new(b"invalid;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            Unit::deserialize(deserializer),
            Error::new(error::Kind::ExpectedUnit, 0, 1)
        );
    }

    #[test]
    fn unit_struct_too_many_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Unit;
        let mut tag = Tag::new(b":;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            Unit::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 2)
        );
    }

    #[test]
    fn unit_struct_unexpected_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Unit;
        let mut tag = Tag::new(b";;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            Unit::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 2)
        );
    }

    #[test]
    fn tuple() {
        let mut tag = Tag::new(b"42:foo::1.2;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(
            <(u64, String, (), f64)>::deserialize(deserializer),
            (42, "foo".to_owned(), (), 1.2)
        );
    }

    #[test]
    fn tuple_too_many_values() {
        let mut tag = Tag::new(b"42:foo::1.2:100:bar::2.4;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            <(u64, String, (), f64)>::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 13)
        );
    }

    #[test]
    fn tuple_unexpected_values() {
        let mut tag = Tag::new(b"42:foo::1.2;100:bar::2.4;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            <(u64, String, (), f64)>::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 13)
        );
    }

    #[test]
    fn tuple_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TupleStruct(u64, String, (), f64);
        let mut tag = Tag::new(b"42:foo::1.2;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(
            TupleStruct::deserialize(deserializer),
            TupleStruct(42, "foo".to_owned(), (), 1.2)
        );
    }

    #[test]
    fn tuple_struct_too_many_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TupleStruct(u64, String, (), f64);
        let mut tag = Tag::new(b"42:foo::1.2:100:bar::2.4;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            TupleStruct::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 13)
        );
    }

    #[test]
    fn tuple_struct_unexpected_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TupleStruct(u64, String, (), f64);
        let mut tag = Tag::new(b"42:foo::1.2;100:bar::2.4;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            TupleStruct::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 13)
        );
    }

    #[test]
    fn map() {
        let mut tag = Tag::new(b"foo:1;bar:2;baz:3;qux:4;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        let mut expected = HashMap::new();
        expected.insert("foo".to_owned(), 1);
        expected.insert("bar".to_owned(), 2);
        expected.insert("baz".to_owned(), 3);
        expected.insert("qux".to_owned(), 4);
        assert_ok_eq!(HashMap::<String, u64>::deserialize(deserializer), expected,);
    }

    #[test]
    fn enum_unit_variant() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Unit {
            Variant,
        }
        let mut tag = Tag::new(b"Variant;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(Unit::deserialize(deserializer), Unit::Variant);
    }

    #[test]
    fn enum_unit_variant_too_many_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Unit {
            Variant,
        }
        let mut tag = Tag::new(b"Variant:42;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

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
        let mut tag = Tag::new(b"Variant;42;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

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
        let mut tag = Tag::new(b"Variant:42;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(Newtype::deserialize(deserializer), Newtype::Variant(42));
    }

    #[test]
    fn enum_newtype_variant_too_many_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Newtype {
            Variant(u64),
        }
        let mut tag = Tag::new(b"Variant:42:foo;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

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
        let mut tag = Tag::new(b"Variant:42;foo;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

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
        let mut tag = Tag::new(b"Variant:42:foo::1.2;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

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
        let mut tag = Tag::new(b"Variant:42:foo::1.2:bar;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

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
        let mut tag = Tag::new(b"Variant:42:foo::1.2;bar;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

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
        let mut tag = Tag::new(b"foo;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_ok_eq!(
            Identifier::deserialize(deserializer),
            Identifier("foo".to_owned())
        );
    }

    #[test]
    fn identifier_invalid() {
        let mut tag = Tag::new(b"\xF0\x9Ffoo;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            Identifier::deserialize(deserializer),
            Error::new(error::Kind::ExpectedIdentifier, 0, 1)
        );
    }

    #[test]
    fn identifier_too_many_values() {
        let mut tag = Tag::new(b"foo:bar;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            Identifier::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 5)
        );
    }

    #[test]
    fn identifier_unexpected_values() {
        let mut tag = Tag::new(b"foo;bar;", 0, 0);
        let values = assert_ok!(tag.next());
        let deserializer = Deserializer::new(tag, values);

        assert_err_eq!(
            Identifier::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 5)
        );
    }
}
