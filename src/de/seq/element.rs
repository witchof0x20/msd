use crate::de::{error, map, parse::Tags, r#enum, r#struct, tuple, Error, Result};
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
        Err(self
            .tags
            .error_at_current_tag(error::Kind::CannotDeserializeAsSelfDescribing))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let value_position = value.position();
        let parsed = value.parse_bool()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_bool(parsed).map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let value_position = value.position();
        let parsed = value.parse_i8()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_i8(parsed).map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let value_position = value.position();
        let parsed = value.parse_i16()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_i16(parsed).map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let value_position = value.position();
        let parsed = value.parse_i32()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_i32(parsed).map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let value_position = value.position();
        let parsed = value.parse_i64()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_i64(parsed).map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
    }

    #[cfg(has_i128)]
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let value_position = value.position();
        let parsed = value.parse_i128()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_i128(parsed).map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let value_position = value.position();
        let parsed = value.parse_u8()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_u8(parsed).map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let value_position = value.position();
        let parsed = value.parse_u16()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_u16(parsed).map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let value_position = value.position();
        let parsed = value.parse_u32()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_u32(parsed).map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let value_position = value.position();
        let parsed = value.parse_u64()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_u64(parsed).map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
    }

    #[cfg(has_i128)]
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let value_position = value.position();
        let parsed = value.parse_u128()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_u128(parsed).map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let value_position = value.position();
        let parsed = value.parse_f32()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_f32(parsed).map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let value_position = value.position();
        let parsed = value.parse_f64()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_f64(parsed).map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let value_position = value.position();
        let parsed = value.parse_char()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_char(parsed).map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let value_position = value.position();
        // Parsed string must be owned, since it removes escaping and comments.
        let parsed = value.parse_string()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_str(&parsed).map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let value_position = value.position();
        let parsed = value.parse_string()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_string(parsed).map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let value_position = value.position();
        // Parsed bytes must be owned, since it removes escaping and comments.
        let parsed = value.parse_byte_buf();
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_bytes(&parsed).map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let value_position = value.position();
        let parsed = value.parse_byte_buf();
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_byte_buf(parsed).map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(self
            .tags
            .error_at_current_tag(error::Kind::CannotDeserializeAsOptionInSeq))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let value_position = value.position();
        value.parse_unit()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_unit().map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut tag = self.tags.next()?;
        let mut values = tag.next()?;
        let value = values.next()?;
        let value_position = value.position();
        value.parse_unit()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_unit().map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
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
        Err(self
            .tags
            .error_at_current_tag(error::Kind::CannotDeserializeNestedSeq))
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
        self,
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
        visitor.visit_map(r#struct::Access::new(self.tags, fields))
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
        let value_position = value.position();
        // Parsed string must be owned, since it removes escaping and comments.
        let parsed = value.parse_identifier()?;
        values.assert_exhausted()?;
        tag.assert_exhausted()?;
        visitor.visit_str(&parsed).map_err(|mut error: Error| {
            error.set_position(value_position);
            error
        })
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(self
            .tags
            .error_at_current_tag(error::Kind::CannotDeserializeAsSelfDescribing))
    }
}

#[cfg(test)]
mod tests {
    use super::Deserializer;
    use crate::de::{error, parse::Tags, Error, Position};
    use claims::{assert_err_eq, assert_ok_eq};
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
            Error::new(error::Kind::ExpectedBool, Position::new(0, 1))
        );
    }

    #[test]
    fn bool_custom_error() {
        #[derive(Debug)]
        struct CustomBool;

        impl<'de> Deserialize<'de> for CustomBool {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomBoolVisitor;

                impl<'de> Visitor<'de> for CustomBoolVisitor {
                    type Value = CustomBool;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_bool<E>(self, _value: bool) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_bool(CustomBoolVisitor)
            }
        }

        let mut tags = Tags::new(b"#true;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomBool::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
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
            Error::new(error::Kind::ExpectedI8, Position::new(0, 1))
        );
    }

    #[test]
    fn i8_too_many_values() {
        let mut tags = Tags::new(b"#42:100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i8::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 4))
        );
    }

    #[test]
    fn i8_unexpected_values() {
        let mut tags = Tags::new(b"#42;100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i8::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 4))
        );
    }

    #[test]
    fn i8_custom_error() {
        #[derive(Debug)]
        struct CustomI8;

        impl<'de> Deserialize<'de> for CustomI8 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomI8Visitor;

                impl<'de> Visitor<'de> for CustomI8Visitor {
                    type Value = CustomI8;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_i8<E>(self, _value: i8) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_i8(CustomI8Visitor)
            }
        }

        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomI8::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
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
            Error::new(error::Kind::ExpectedI16, Position::new(0, 1))
        );
    }

    #[test]
    fn i16_too_many_values() {
        let mut tags = Tags::new(b"#42:100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i16::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 4))
        );
    }

    #[test]
    fn i16_unexpected_values() {
        let mut tags = Tags::new(b"#42;100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i16::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 4))
        );
    }

    #[test]
    fn i16_custom_error() {
        #[derive(Debug)]
        struct CustomI16;

        impl<'de> Deserialize<'de> for CustomI16 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomI16Visitor;

                impl<'de> Visitor<'de> for CustomI16Visitor {
                    type Value = CustomI16;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_i16<E>(self, _value: i16) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_i16(CustomI16Visitor)
            }
        }

        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomI16::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
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
            Error::new(error::Kind::ExpectedI32, Position::new(0, 1))
        );
    }

    #[test]
    fn i32_too_many_values() {
        let mut tags = Tags::new(b"#42:100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 4))
        );
    }

    #[test]
    fn i32_unexpected_values() {
        let mut tags = Tags::new(b"#42;100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 4))
        );
    }

    #[test]
    fn i32_custom_error() {
        #[derive(Debug)]
        struct CustomI32;

        impl<'de> Deserialize<'de> for CustomI32 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomI32Visitor;

                impl<'de> Visitor<'de> for CustomI32Visitor {
                    type Value = CustomI32;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_i32<E>(self, _value: i32) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_i32(CustomI32Visitor)
            }
        }

        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomI32::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
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
            Error::new(error::Kind::ExpectedI64, Position::new(0, 1))
        );
    }

    #[test]
    fn i64_too_many_values() {
        let mut tags = Tags::new(b"#42:100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 4))
        );
    }

    #[test]
    fn i64_unexpected_values() {
        let mut tags = Tags::new(b"#42;100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 4))
        );
    }

    #[test]
    fn i64_custom_error() {
        #[derive(Debug)]
        struct CustomI64;

        impl<'de> Deserialize<'de> for CustomI64 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomI64Visitor;

                impl<'de> Visitor<'de> for CustomI64Visitor {
                    type Value = CustomI64;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_i64<E>(self, _value: i64) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_i64(CustomI64Visitor)
            }
        }

        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomI64::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
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
            Error::new(error::Kind::ExpectedI128, Position::new(0, 1))
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128_too_many_values() {
        let mut tags = Tags::new(b"#42:100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i128::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 4))
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128_unexpected_values() {
        let mut tags = Tags::new(b"#42;100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            i128::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 4))
        );
    }

    #[test]
    fn i128_custom_error() {
        #[derive(Debug)]
        struct CustomI128;

        impl<'de> Deserialize<'de> for CustomI128 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomI128Visitor;

                impl<'de> Visitor<'de> for CustomI128Visitor {
                    type Value = CustomI128;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_i128<E>(self, _value: i128) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_i128(CustomI128Visitor)
            }
        }

        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomI128::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
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
            Error::new(error::Kind::ExpectedU8, Position::new(0, 1))
        );
    }

    #[test]
    fn u8_too_many_values() {
        let mut tags = Tags::new(b"#42:100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u8::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 4))
        );
    }

    #[test]
    fn u8_unexpected_values() {
        let mut tags = Tags::new(b"#42;100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u8::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 4))
        );
    }

    #[test]
    fn u8_custom_error() {
        #[derive(Debug)]
        struct CustomU8;

        impl<'de> Deserialize<'de> for CustomU8 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomU8Visitor;

                impl<'de> Visitor<'de> for CustomU8Visitor {
                    type Value = CustomU8;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_u8<E>(self, _value: u8) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_u8(CustomU8Visitor)
            }
        }

        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomU8::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
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
            Error::new(error::Kind::ExpectedU16, Position::new(0, 1))
        );
    }

    #[test]
    fn u16_too_many_values() {
        let mut tags = Tags::new(b"#42:100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u16::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 4))
        );
    }

    #[test]
    fn u16_unexpected_values() {
        let mut tags = Tags::new(b"#42;100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u16::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 4))
        );
    }

    #[test]
    fn u16_custom_error() {
        #[derive(Debug)]
        struct CustomU16;

        impl<'de> Deserialize<'de> for CustomU16 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomU16Visitor;

                impl<'de> Visitor<'de> for CustomU16Visitor {
                    type Value = CustomU16;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_u16<E>(self, _value: u16) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_u16(CustomU16Visitor)
            }
        }

        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomU16::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
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
            Error::new(error::Kind::ExpectedU32, Position::new(0, 1))
        );
    }

    #[test]
    fn u32_too_many_values() {
        let mut tags = Tags::new(b"#42:100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 4))
        );
    }

    #[test]
    fn u32_unexpected_values() {
        let mut tags = Tags::new(b"#42;100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 4))
        );
    }

    #[test]
    fn u32_custom_error() {
        #[derive(Debug)]
        struct CustomU32;

        impl<'de> Deserialize<'de> for CustomU32 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomU32Visitor;

                impl<'de> Visitor<'de> for CustomU32Visitor {
                    type Value = CustomU32;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_u32<E>(self, _value: u32) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_u32(CustomU32Visitor)
            }
        }

        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomU32::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
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
            Error::new(error::Kind::ExpectedU64, Position::new(0, 1))
        );
    }

    #[test]
    fn u64_too_many_values() {
        let mut tags = Tags::new(b"#42:100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 4))
        );
    }

    #[test]
    fn u64_unexpected_values() {
        let mut tags = Tags::new(b"#42;100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 4))
        );
    }

    #[test]
    fn u64_custom_error() {
        #[derive(Debug)]
        struct CustomU64;

        impl<'de> Deserialize<'de> for CustomU64 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomU64Visitor;

                impl<'de> Visitor<'de> for CustomU64Visitor {
                    type Value = CustomU64;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_u64<E>(self, _value: u64) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_u64(CustomU64Visitor)
            }
        }

        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomU64::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
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
            Error::new(error::Kind::ExpectedU128, Position::new(0, 1))
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128_too_many_values() {
        let mut tags = Tags::new(b"#42:100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u128::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 4))
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128_unexpected_values() {
        let mut tags = Tags::new(b"#42;100;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            u128::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 4))
        );
    }

    #[test]
    fn u128_custom_error() {
        #[derive(Debug)]
        struct CustomU128;

        impl<'de> Deserialize<'de> for CustomU128 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomU128Visitor;

                impl<'de> Visitor<'de> for CustomU128Visitor {
                    type Value = CustomU128;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_u128<E>(self, _value: u128) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_u128(CustomU128Visitor)
            }
        }

        let mut tags = Tags::new(b"#42;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomU128::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
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
            Error::new(error::Kind::ExpectedF32, Position::new(0, 1))
        );
    }

    #[test]
    fn f32_too_many_values() {
        let mut tags = Tags::new(b"#42.9:1.2;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            f32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 6))
        );
    }

    #[test]
    fn f32_unexpected_values() {
        let mut tags = Tags::new(b"#42.9;1.2;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            f32::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 6))
        );
    }

    #[test]
    fn f32_custom_error() {
        #[derive(Debug)]
        struct CustomF32;

        impl<'de> Deserialize<'de> for CustomF32 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomF32Visitor;

                impl<'de> Visitor<'de> for CustomF32Visitor {
                    type Value = CustomF32;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_f32<E>(self, _value: f32) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_f32(CustomF32Visitor)
            }
        }

        let mut tags = Tags::new(b"#42.9;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomF32::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
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
            Error::new(error::Kind::ExpectedF64, Position::new(0, 1))
        );
    }

    #[test]
    fn f64_too_many_values() {
        let mut tags = Tags::new(b"#42.9:1.2;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            f64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 6))
        );
    }

    #[test]
    fn f64_unexpected_values() {
        let mut tags = Tags::new(b"#42.9;1.2;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            f64::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 6))
        );
    }

    #[test]
    fn f64_custom_error() {
        #[derive(Debug)]
        struct CustomF64;

        impl<'de> Deserialize<'de> for CustomF64 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomF64Visitor;

                impl<'de> Visitor<'de> for CustomF64Visitor {
                    type Value = CustomF64;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_f64<E>(self, _value: f64) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_f64(CustomF64Visitor)
            }
        }

        let mut tags = Tags::new(b"#42.9;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomF64::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
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
            Error::new(error::Kind::ExpectedChar, Position::new(0, 1))
        );
    }

    #[test]
    fn char_too_many_values() {
        let mut tags = Tags::new(b"#a:b;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            char::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 3))
        );
    }

    #[test]
    fn char_unexpected_values() {
        let mut tags = Tags::new(b"#a;b;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            char::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 3))
        );
    }

    #[test]
    fn char_custom_error() {
        #[derive(Debug)]
        struct CustomChar;

        impl<'de> Deserialize<'de> for CustomChar {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomCharVisitor;

                impl<'de> Visitor<'de> for CustomCharVisitor {
                    type Value = CustomChar;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_char<E>(self, _value: char) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_char(CustomCharVisitor)
            }
        }

        let mut tags = Tags::new(b"#a;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomChar::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
        );
    }

    #[derive(Debug, PartialEq)]
    struct Str(String);

    impl<'de> Deserialize<'de> for Str {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct StrVisitor;

            impl<'de> Visitor<'de> for StrVisitor {
                type Value = Str;

                fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                    unimplemented!()
                }

                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Ok(Str(value.to_owned()))
                }
            }

            deserializer.deserialize_str(StrVisitor)
        }
    }

    #[test]
    fn str() {
        let mut tags = Tags::new(b"#foo;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(Str::deserialize(deserializer), Str("foo".to_owned()));
    }

    #[test]
    fn str_invalid() {
        let mut tags = Tags::new(b"#\xF0\x9Ffoo;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Str::deserialize(deserializer),
            Error::new(error::Kind::ExpectedString, Position::new(0, 1))
        );
    }

    #[test]
    fn str_too_many_values() {
        let mut tags = Tags::new(b"#foo:bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Str::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 5))
        );
    }

    #[test]
    fn str_unexpected_values() {
        let mut tags = Tags::new(b"#foo;bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Str::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 5))
        );
    }

    #[test]
    fn str_custom_error() {
        #[derive(Debug)]
        struct CustomStr;

        impl<'de> Deserialize<'de> for CustomStr {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomStrVisitor;

                impl<'de> Visitor<'de> for CustomStrVisitor {
                    type Value = CustomStr;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_str(CustomStrVisitor)
            }
        }

        let mut tags = Tags::new(b"#foo;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomStr::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
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
            Error::new(error::Kind::ExpectedString, Position::new(0, 1))
        );
    }

    #[test]
    fn string_too_many_values() {
        let mut tags = Tags::new(b"#foo:bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            String::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 5))
        );
    }

    #[test]
    fn string_unexpected_values() {
        let mut tags = Tags::new(b"#foo;bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            String::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 5))
        );
    }

    #[test]
    fn string_custom_error() {
        #[derive(Debug)]
        struct CustomString;

        impl<'de> Deserialize<'de> for CustomString {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomStringVisitor;

                impl<'de> Visitor<'de> for CustomStringVisitor {
                    type Value = CustomString;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_string(CustomStringVisitor)
            }
        }

        let mut tags = Tags::new(b"#foo;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomString::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
        );
    }

    #[derive(Debug, PartialEq)]
    struct Bytes(Vec<u8>);

    impl<'de> Deserialize<'de> for Bytes {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct BytesVisitor;

            impl<'de> Visitor<'de> for BytesVisitor {
                type Value = Bytes;

                fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                    unimplemented!()
                }

                fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Ok(Bytes(value.to_owned()))
                }
            }

            deserializer.deserialize_bytes(BytesVisitor)
        }
    }

    #[test]
    fn bytes() {
        let mut tags = Tags::new(b"#foo;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_ok_eq!(Bytes::deserialize(deserializer), Bytes(b"foo".to_vec()));
    }

    #[test]
    fn bytes_too_many_values() {
        let mut tags = Tags::new(b"#foo:bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Bytes::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 5))
        );
    }

    #[test]
    fn bytes_unexpected_values() {
        let mut tags = Tags::new(b"#foo;bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Bytes::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 5))
        );
    }

    #[test]
    fn bytes_custom_error() {
        #[derive(Debug)]
        struct CustomBytes;

        impl<'de> Deserialize<'de> for CustomBytes {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomBytesVisitor;

                impl<'de> Visitor<'de> for CustomBytesVisitor {
                    type Value = CustomBytes;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_bytes<E>(self, _value: &[u8]) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_bytes(CustomBytesVisitor)
            }
        }

        let mut tags = Tags::new(b"#foo;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomBytes::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
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
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 5))
        );
    }

    #[test]
    fn byte_buf_unexpected_values() {
        let mut tags = Tags::new(b"#foo;bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            ByteBuf::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 5))
        );
    }

    #[test]
    fn byte_buf_custom_error() {
        #[derive(Debug)]
        struct CustomByteBuf;

        impl<'de> Deserialize<'de> for CustomByteBuf {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomByteBufVisitor;

                impl<'de> Visitor<'de> for CustomByteBufVisitor {
                    type Value = CustomByteBuf;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_byte_buf<E>(self, _value: Vec<u8>) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_byte_buf(CustomByteBufVisitor)
            }
        }

        let mut tags = Tags::new(b"#foo;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomByteBuf::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
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
            Error::new(error::Kind::ExpectedUnit, Position::new(0, 1))
        );
    }

    #[test]
    fn unit_too_many_values() {
        let mut tags = Tags::new(b"#:;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            <()>::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 2))
        );
    }

    #[test]
    fn unit_unexpected_values() {
        let mut tags = Tags::new(b"#;;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            <()>::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 2))
        );
    }

    #[test]
    fn unit_custom_error() {
        #[derive(Debug)]
        struct CustomUnit;

        impl<'de> Deserialize<'de> for CustomUnit {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomUnitVisitor;

                impl<'de> Visitor<'de> for CustomUnitVisitor {
                    type Value = CustomUnit;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_unit<E>(self) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_unit(CustomUnitVisitor)
            }
        }

        let mut tags = Tags::new(b"#;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomUnit::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
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
            Error::new(error::Kind::ExpectedUnit, Position::new(0, 1))
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
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 2))
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
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 2))
        );
    }

    #[test]
    fn unit_struct_custom_error() {
        #[derive(Debug)]
        struct CustomUnitStruct;

        impl<'de> Deserialize<'de> for CustomUnitStruct {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomUnitStructVisitor;

                impl<'de> Visitor<'de> for CustomUnitStructVisitor {
                    type Value = CustomUnitStruct;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_unit<E>(self) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_unit_struct("CustomUnitStruct", CustomUnitStructVisitor)
            }
        }

        let mut tags = Tags::new(b"#;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomUnitStruct::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
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
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 13))
        );
    }

    #[test]
    fn tuple_unexpected_values() {
        let mut tags = Tags::new(b"#42:foo::1.2;bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            <(u64, String, (), f64)>::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 13))
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
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 13))
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
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 13))
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
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 9))
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
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 9))
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
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 12))
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
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 12))
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
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 21))
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
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 21))
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
            Error::new(error::Kind::ExpectedIdentifier, Position::new(0, 1))
        );
    }

    #[test]
    fn identifier_too_many_values() {
        let mut tags = Tags::new(b"#foo:bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Identifier::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValue, Position::new(0, 5))
        );
    }

    #[test]
    fn identifier_unexpected_values() {
        let mut tags = Tags::new(b"#foo;bar;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Identifier::deserialize(deserializer),
            Error::new(error::Kind::UnexpectedValues, Position::new(0, 5))
        );
    }

    #[test]
    fn identifier_custom_error() {
        #[derive(Debug)]
        struct CustomIdentifier;

        impl<'de> Deserialize<'de> for CustomIdentifier {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct CustomIdentifierVisitor;

                impl<'de> Visitor<'de> for CustomIdentifierVisitor {
                    type Value = CustomIdentifier;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }

                    fn visit_str<E>(self, _value: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Err(de::Error::custom("foo"))
                    }
                }

                deserializer.deserialize_identifier(CustomIdentifierVisitor)
            }
        }

        let mut tags = Tags::new(b"#foo;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            CustomIdentifier::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(0, 1))
        );
    }

    #[test]
    fn any() {
        #[derive(Debug)]
        struct Any;

        impl<'de> Deserialize<'de> for Any {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct AnyVisitor;

                impl<'de> Visitor<'de> for AnyVisitor {
                    type Value = Any;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }
                }

                deserializer.deserialize_any(AnyVisitor)
            }
        }

        let mut tags = Tags::new(b"#foo;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Any::deserialize(deserializer),
            Error::new(
                error::Kind::CannotDeserializeAsSelfDescribing,
                Position::new(0, 1)
            )
        );
    }

    #[test]
    fn ignored_any() {
        #[derive(Debug)]
        struct IgnoredAny;

        impl<'de> Deserialize<'de> for IgnoredAny {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct IgnoredAnyVisitor;

                impl<'de> Visitor<'de> for IgnoredAnyVisitor {
                    type Value = IgnoredAny;

                    fn expecting(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                        unimplemented!()
                    }
                }

                deserializer.deserialize_ignored_any(IgnoredAnyVisitor)
            }
        }

        let mut tags = Tags::new(b"#foo;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            IgnoredAny::deserialize(deserializer),
            Error::new(
                error::Kind::CannotDeserializeAsSelfDescribing,
                Position::new(0, 1)
            )
        );
    }

    #[test]
    fn option() {
        let mut tags = Tags::new(b"#foo;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Option::<()>::deserialize(deserializer),
            Error::new(
                error::Kind::CannotDeserializeAsOptionInSeq,
                Position::new(0, 1)
            )
        );
    }

    #[test]
    fn seq() {
        let mut tags = Tags::new(b"#foo;".as_slice());
        let deserializer = Deserializer::new(&mut tags);

        assert_err_eq!(
            Vec::<()>::deserialize(deserializer),
            Error::new(error::Kind::CannotDeserializeNestedSeq, Position::new(0, 1))
        );
    }
}
