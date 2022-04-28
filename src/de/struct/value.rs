use crate::de::{
    map,
    parse::{StoredTag, StoredValues, Tags},
    r#enum, seq, tuple, Error, Result,
};
use serde::{de, de::Visitor};
use std::io::Read;

pub(in super::super) struct Deserializer<'a, R> {
    field: &'static str,
    tags: &'a mut Tags<R>,

    tag: StoredTag,
    values: StoredValues,
}

impl<'a, R> Deserializer<'a, R> {
    pub(in super::super) fn new(
        field: &'static str,
        tags: &'a mut Tags<R>,
        tag: StoredTag,
        values: StoredValues,
    ) -> Self {
        Self {
            field,
            tags,
            tag,
            values,
        }
    }
}

impl<'a, 'de, R> de::Deserializer<'de> for Deserializer<'a, R>
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
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        let parsed = value.parse_bool()?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        visitor.visit_bool(parsed)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        let parsed = value.parse_i8()?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        visitor.visit_i8(parsed)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        let parsed = value.parse_i16()?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        visitor.visit_i16(parsed)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        let parsed = value.parse_i32()?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        visitor.visit_i32(parsed)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        let parsed = value.parse_i64()?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        visitor.visit_i64(parsed)
    }

    #[cfg(has_i128)]
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        let parsed = value.parse_i128()?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        visitor.visit_i128(parsed)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        let parsed = value.parse_u8()?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        visitor.visit_u8(parsed)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        let parsed = value.parse_u16()?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        visitor.visit_u16(parsed)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        let parsed = value.parse_u32()?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        visitor.visit_u32(parsed)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        let parsed = value.parse_u64()?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        visitor.visit_u64(parsed)
    }

    #[cfg(has_i128)]
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        let parsed = value.parse_u128()?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        visitor.visit_u128(parsed)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        let parsed = value.parse_f32()?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        visitor.visit_f32(parsed)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        let parsed = value.parse_f64()?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        visitor.visit_f64(parsed)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        let parsed = value.parse_char()?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        visitor.visit_char(parsed)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        let parsed = value.parse_string()?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        visitor.visit_str(&parsed)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        let parsed = value.parse_string()?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        visitor.visit_string(parsed)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        let parsed = value.parse_byte_buf();
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        visitor.visit_bytes(&parsed)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        let parsed = value.parse_byte_buf();
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        visitor.visit_byte_buf(parsed)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // If a field is present, it must be a `Some` value.
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        value.parse_unit()?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        value.parse_unit()?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = unsafe { self.tag.into_tag() };
        tag.reset();
        unsafe { self.tags.revisit(tag.into_stored()) };

        visitor.visit_seq(seq::field::Access::new(self.field, self.tags))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let result = visitor.visit_seq(tuple::Access::new(&mut values, len))?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
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
        let mut values = unsafe { self.values.into_values() };
        let result = visitor.visit_seq(tuple::Access::new(&mut values, len))?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        Ok(result)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = unsafe { self.tag.into_tag() };
        // SAFETY: `self.values` references the same buffer that `self.tag` references.
        unsafe { tag.revisit(self.values.into_values()) };
        let result = visitor.visit_map(map::field::Access::new(&mut tag))?;
        tag.assert_exhausted()?;
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
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let result = visitor.visit_enum(r#enum::Access::new(&mut values))?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
        Ok(result)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut values = unsafe { self.values.into_values() };
        let value = values.next()?;
        let parsed = value.parse_identifier()?;
        values.assert_exhausted()?;
        unsafe { self.tag.into_tag() }.assert_exhausted()?;
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
    use claim::{assert_err_eq, assert_ok, assert_ok_eq};
    use serde::{de, de::Visitor, Deserialize};
    use serde_bytes::ByteBuf;
    use serde_derive::Deserialize;
    use std::{collections::HashMap, fmt};

    #[test]
    fn bool_true() {
        let mut tags = Tags::new(b"#foo:true;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(bool::deserialize(deserializer), true);
    }

    #[test]
    fn bool_false() {
        let mut tags = Tags::new(b"#foo:false;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(bool::deserialize(deserializer), false);
    }

    #[test]
    fn bool_invalid() {
        let mut tags = Tags::new(b"#foo:invalid;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            bool::deserialize(deserializer),
            Error::new(error::Kind::ExpectedBool, 0, 5)
        );
    }

    #[test]
    fn bool_too_many_values() {
        let mut tags = Tags::new(b"#foo:true:true;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            bool::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 10)
        );
    }

    #[test]
    fn bool_unexpected_values() {
        let mut tags = Tags::new(b"#foo:true;true;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            bool::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 10)
        );
    }

    #[test]
    fn i8() {
        let mut tags = Tags::new(b"#foo:42;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(i8::deserialize(deserializer), 42);
    }

    #[test]
    fn i8_invalid() {
        let mut tags = Tags::new(b"#foo:invalid;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            i8::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI8, 0, 5)
        );
    }

    #[test]
    fn i8_too_many_values() {
        let mut tags = Tags::new(b"#foo:42:100;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            i8::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 8)
        );
    }

    #[test]
    fn i8_unexpected_values() {
        let mut tags = Tags::new(b"#foo:42;100;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            i8::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 8)
        );
    }

    #[test]
    fn i16() {
        let mut tags = Tags::new(b"#foo:42;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(i16::deserialize(deserializer), 42);
    }

    #[test]
    fn i16_invalid() {
        let mut tags = Tags::new(b"#foo:invalid;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            i16::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI16, 0, 5)
        );
    }

    #[test]
    fn i16_too_many_values() {
        let mut tags = Tags::new(b"#foo:42:100;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            i16::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 8)
        );
    }

    #[test]
    fn i16_unexpected_values() {
        let mut tags = Tags::new(b"#foo:42;100;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            i16::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 8)
        );
    }

    #[test]
    fn i32() {
        let mut tags = Tags::new(b"#foo:42;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(i32::deserialize(deserializer), 42);
    }

    #[test]
    fn i32_invalid() {
        let mut tags = Tags::new(b"#foo:invalid;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            i32::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI32, 0, 5)
        );
    }

    #[test]
    fn i32_too_many_values() {
        let mut tags = Tags::new(b"#foo:42:100;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            i32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 8)
        );
    }

    #[test]
    fn i32_unexpected_values() {
        let mut tags = Tags::new(b"#foo:42;100;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            i32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 8)
        );
    }

    #[test]
    fn i64() {
        let mut tags = Tags::new(b"#foo:42;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(i64::deserialize(deserializer), 42);
    }

    #[test]
    fn i64_invalid() {
        let mut tags = Tags::new(b"#foo:invalid;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            i64::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI64, 0, 5)
        );
    }

    #[test]
    fn i64_too_many_values() {
        let mut tags = Tags::new(b"#foo:42:100;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            i64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 8)
        );
    }

    #[test]
    fn i64_unexpected_values() {
        let mut tags = Tags::new(b"#foo:42;100;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            i64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 8)
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128() {
        let mut tags = Tags::new(b"#foo:42;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(i128::deserialize(deserializer), 42);
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128_invalid() {
        let mut tags = Tags::new(b"#foo:invalid;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            i128::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI128, 0, 5)
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128_too_many_values() {
        let mut tags = Tags::new(b"#foo:42:100;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            i128::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 8)
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128_unexpected_values() {
        let mut tags = Tags::new(b"#foo:42;100;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            i128::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 8)
        );
    }

    #[test]
    fn u8() {
        let mut tags = Tags::new(b"#foo:42;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(u8::deserialize(deserializer), 42);
    }

    #[test]
    fn u8_invalid() {
        let mut tags = Tags::new(b"#foo:invalid;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            u8::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU8, 0, 5)
        );
    }

    #[test]
    fn u8_too_many_values() {
        let mut tags = Tags::new(b"#foo:42:100;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            u8::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 8)
        );
    }

    #[test]
    fn u8_unexpected_values() {
        let mut tags = Tags::new(b"#foo:42;100;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            u8::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 8)
        );
    }

    #[test]
    fn u16() {
        let mut tags = Tags::new(b"#foo:42;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(u16::deserialize(deserializer), 42);
    }

    #[test]
    fn u16_invalid() {
        let mut tags = Tags::new(b"#foo:invalid;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            u16::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU16, 0, 5)
        );
    }

    #[test]
    fn u16_too_many_values() {
        let mut tags = Tags::new(b"#foo:42:100;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            u16::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 8)
        );
    }

    #[test]
    fn u16_unexpected_values() {
        let mut tags = Tags::new(b"#foo:42;100;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            u16::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 8)
        );
    }

    #[test]
    fn u32() {
        let mut tags = Tags::new(b"#foo:42;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(u32::deserialize(deserializer), 42);
    }

    #[test]
    fn u32_invalid() {
        let mut tags = Tags::new(b"#foo:invalid;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            u32::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU32, 0, 5)
        );
    }

    #[test]
    fn u32_too_many_values() {
        let mut tags = Tags::new(b"#foo:42:100;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            u32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 8)
        );
    }

    #[test]
    fn u32_unexpected_values() {
        let mut tags = Tags::new(b"#foo:42;100;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            u32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 8)
        );
    }

    #[test]
    fn u64() {
        let mut tags = Tags::new(b"#foo:42;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(u64::deserialize(deserializer), 42);
    }

    #[test]
    fn u64_invalid() {
        let mut tags = Tags::new(b"#foo:invalid;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            u64::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU64, 0, 5)
        );
    }

    #[test]
    fn u64_too_many_values() {
        let mut tags = Tags::new(b"#foo:42:100;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            u64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 8)
        );
    }

    #[test]
    fn u64_unexpected_values() {
        let mut tags = Tags::new(b"#foo:42;100;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            u64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 8)
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128() {
        let mut tags = Tags::new(b"#foo:42;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(u128::deserialize(deserializer), 42);
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128_invalid() {
        let mut tags = Tags::new(b"#foo:invalid;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            u128::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU128, 0, 5)
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128_too_many_values() {
        let mut tags = Tags::new(b"#foo:42:100;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            u128::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 8)
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128_unexpected_values() {
        let mut tags = Tags::new(b"#foo:42;100;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            u128::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 8)
        );
    }

    #[test]
    fn f32() {
        let mut tags = Tags::new(b"#foo:42.9;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(f32::deserialize(deserializer), 42.9);
    }

    #[test]
    fn f32_invalid() {
        let mut tags = Tags::new(b"#foo:invalid;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            f32::deserialize(deserializer),
            Error::new(error::Kind::ExpectedF32, 0, 5)
        );
    }

    #[test]
    fn f32_too_many_values() {
        let mut tags = Tags::new(b"#foo:42.9:1.2;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            f32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 10)
        );
    }

    #[test]
    fn f32_unexpected_values() {
        let mut tags = Tags::new(b"#foo:42.9;1.2;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            f32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 10)
        );
    }

    #[test]
    fn f64() {
        let mut tags = Tags::new(b"#foo:42.9;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(f64::deserialize(deserializer), 42.9);
    }

    #[test]
    fn f64_invalid() {
        let mut tags = Tags::new(b"#foo:invalid;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            f64::deserialize(deserializer),
            Error::new(error::Kind::ExpectedF64, 0, 5)
        );
    }

    #[test]
    fn f64_too_many_values() {
        let mut tags = Tags::new(b"#foo:42.9:1.2;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            f64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 10)
        );
    }

    #[test]
    fn f64_unexpected_values() {
        let mut tags = Tags::new(b"#foo:42.9;1.2;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            f64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 10)
        );
    }

    #[test]
    fn char() {
        let mut tags = Tags::new(b"#foo:a;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(char::deserialize(deserializer), 'a');
    }

    #[test]
    fn char_invalid() {
        let mut tags = Tags::new(b"#foo:invalid;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            char::deserialize(deserializer),
            Error::new(error::Kind::ExpectedChar, 0, 5)
        );
    }

    #[test]
    fn char_too_many_values() {
        let mut tags = Tags::new(b"#foo:a:b;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            char::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 7)
        );
    }

    #[test]
    fn char_unexpected_values() {
        let mut tags = Tags::new(b"#foo:a;b;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            char::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 7)
        );
    }

    #[test]
    fn string() {
        let mut tags = Tags::new(b"#foo:foo;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(String::deserialize(deserializer), "foo");
    }

    #[test]
    fn string_invalid() {
        let mut tags = Tags::new(b"#foo:\xF0\x9Ffoo;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            String::deserialize(deserializer),
            Error::new(error::Kind::ExpectedString, 0, 5)
        );
    }

    #[test]
    fn string_too_many_values() {
        let mut tags = Tags::new(b"#foo:foo:bar;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            String::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 9)
        );
    }

    #[test]
    fn string_unexpected_values() {
        let mut tags = Tags::new(b"#foo:foo;bar;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            String::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 9)
        );
    }

    #[test]
    fn bytes() {
        let mut tags = Tags::new(b"#foo:foo;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(ByteBuf::deserialize(deserializer), b"foo");
    }

    #[test]
    fn bytes_too_many_values() {
        let mut tags = Tags::new(b"#foo:foo:bar;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            ByteBuf::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 9)
        );
    }

    #[test]
    fn bytes_unexpected_values() {
        let mut tags = Tags::new(b"#foo:foo;bar;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            ByteBuf::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 9)
        );
    }

    #[test]
    fn some() {
        let mut tags = Tags::new(b"#foo:42;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(Option::<u64>::deserialize(deserializer), Some(42));
    }

    #[test]
    fn unit() {
        let mut tags = Tags::new(b"#foo:;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(<()>::deserialize(deserializer), ());
    }

    #[test]
    fn unit_invalid() {
        let mut tags = Tags::new(b"#foo:invalid;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            <()>::deserialize(deserializer),
            Error::new(error::Kind::ExpectedUnit, 0, 5)
        );
    }

    #[test]
    fn unit_too_many_values() {
        let mut tags = Tags::new(b"#foo::;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            <()>::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 6)
        );
    }

    #[test]
    fn unit_unexpected_values() {
        let mut tags = Tags::new(b"#foo:;;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            <()>::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 6)
        );
    }

    #[test]
    fn unit_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Unit;
        let mut tags = Tags::new(b"#foo:;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(Unit::deserialize(deserializer), Unit);
    }

    #[test]
    fn unit_struct_invalid() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Unit;
        let mut tags = Tags::new(b"#foo:invalid;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            Unit::deserialize(deserializer),
            Error::new(error::Kind::ExpectedUnit, 0, 5)
        );
    }

    #[test]
    fn unit_struct_too_many_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Unit;
        let mut tags = Tags::new(b"#foo::;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            Unit::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 6)
        );
    }

    #[test]
    fn unit_struct_unexpected_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Unit;
        let mut tags = Tags::new(b"#foo:;;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            Unit::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 6)
        );
    }

    #[test]
    fn seq() {
        let mut tags = Tags::new(b"#foo:1;\n#foo:2;\n#foo:3;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(Vec::<u64>::deserialize(deserializer), vec![1, 2, 3]);
    }

    #[test]
    fn tuple() {
        let mut tags = Tags::new(b"#foo:42:foo::1.2;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(
            <(u64, String, (), f64)>::deserialize(deserializer),
            (42, "foo".to_owned(), (), 1.2)
        );
    }

    #[test]
    fn tuple_too_many_values() {
        let mut tags = Tags::new(b"#foo:42:foo::1.2:100:bar::2.4;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            <(u64, String, (), f64)>::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 17)
        );
    }

    #[test]
    fn tuple_unexpected_values() {
        let mut tags = Tags::new(b"#foo:42:foo::1.2;100:bar::2.4;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            <(u64, String, (), f64)>::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 17)
        );
    }

    #[test]
    fn tuple_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TupleStruct(u64, String, (), f64);
        let mut tags = Tags::new(b"#foo:42:foo::1.2;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(
            TupleStruct::deserialize(deserializer),
            TupleStruct(42, "foo".to_owned(), (), 1.2)
        );
    }

    #[test]
    fn tuple_struct_too_many_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TupleStruct(u64, String, (), f64);
        let mut tags = Tags::new(b"#foo:42:foo::1.2:100:bar::2.4;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            TupleStruct::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 17)
        );
    }

    #[test]
    fn tuple_struct_unexpected_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TupleStruct(u64, String, (), f64);
        let mut tags = Tags::new(b"#foo:42:foo::1.2;100:bar::2.4;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            TupleStruct::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 17)
        );
    }

    #[test]
    fn map() {
        let mut tags = Tags::new(b"#foo:foo:1;bar:2;baz:3;qux:4;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

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
        let mut tags = Tags::new(b"#foo:Variant;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(Unit::deserialize(deserializer), Unit::Variant);
    }

    #[test]
    fn enum_unit_variant_too_many_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Unit {
            Variant,
        }
        let mut tags = Tags::new(b"#foo:Variant:42;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            Unit::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 13)
        );
    }

    #[test]
    fn enum_unit_variant_unexpected_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Unit {
            Variant,
        }
        let mut tags = Tags::new(b"#foo:Variant;42;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            Unit::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 13)
        );
    }

    #[test]
    fn enum_newtype_variant() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Newtype {
            Variant(u64),
        }
        let mut tags = Tags::new(b"#foo:Variant:42;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(Newtype::deserialize(deserializer), Newtype::Variant(42));
    }

    #[test]
    fn enum_newtype_variant_too_many_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Newtype {
            Variant(u64),
        }
        let mut tags = Tags::new(b"#foo:Variant:42:foo;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            Newtype::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 16)
        );
    }

    #[test]
    fn enum_newtype_variant_unexpected_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Newtype {
            Variant(u64),
        }
        let mut tags = Tags::new(b"#foo:Variant:42;foo;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            Newtype::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 16)
        );
    }

    #[test]
    fn enum_tuple_variant() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Tuple {
            Variant(u64, String, (), f64),
        }
        let mut tags = Tags::new(b"#foo:Variant:42:foo::1.2;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

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
        let mut tags = Tags::new(b"#foo:Variant:42:foo::1.2:bar;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            Tuple::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 25)
        );
    }

    #[test]
    fn enum_tuple_variant_unexpected_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Tuple {
            Variant(u64, String, (), f64),
        }
        let mut tags = Tags::new(b"#foo:Variant:42:foo::1.2;bar;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            Tuple::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 25)
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
        let mut tags = Tags::new(b"#foo:foo;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_ok_eq!(
            Identifier::deserialize(deserializer),
            Identifier("foo".to_owned())
        );
    }

    #[test]
    fn identifier_invalid() {
        let mut tags = Tags::new(b"#foo:\xF0\x9Ffoo;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            Identifier::deserialize(deserializer),
            Error::new(error::Kind::ExpectedIdentifier, 0, 5)
        );
    }

    #[test]
    fn identifier_too_many_values() {
        let mut tags = Tags::new(b"#foo:foo:bar;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            Identifier::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, 0, 9)
        );
    }

    #[test]
    fn identifier_unexpected_values() {
        let mut tags = Tags::new(b"#foo:foo;bar;\n".as_slice());
        let mut tag = assert_ok!(tags.next());
        let mut values = assert_ok!(tag.next());
        let _field = assert_ok!(values.next());
        let stored_tag = tag.into_stored();
        let stored_values = values.into_stored();
        let deserializer = Deserializer::new("foo", &mut tags, stored_tag, stored_values);

        assert_err_eq!(
            Identifier::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, 0, 9)
        );
    }
}
