use crate::de::{parse::Values, r#enum, Error, Result};
use serde::de::Visitor;

pub(in crate::de) struct Deserializer<'a, 'b> {
    values: &'a mut Values<'b>,
}

impl<'a, 'b> Deserializer<'a, 'b> {
    pub(in crate::de) fn new(values: &'a mut Values<'b>) -> Self {
        Self { values }
    }
}

impl<'a, 'b, 'de> serde::Deserializer<'de> for Deserializer<'a, 'b> {
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
        visitor.visit_bool(self.values.next()?.parse_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.values.next()?.parse_i8()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.values.next()?.parse_i16()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.values.next()?.parse_i32()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.values.next()?.parse_i64()?)
    }

    #[cfg(has_i128)]
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i128(self.values.next()?.parse_i128()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.values.next()?.parse_u8()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.values.next()?.parse_u16()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.values.next()?.parse_u32()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.values.next()?.parse_u64()?)
    }

    #[cfg(has_i128)]
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u128(self.values.next()?.parse_u128()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.values.next()?.parse_f32()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.values.next()?.parse_f64()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_char(self.values.next()?.parse_char()?)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(&self.values.next()?.parse_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.values.next()?.parse_string()?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bytes(&self.values.next()?.parse_byte_buf())
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_byte_buf(self.values.next()?.parse_byte_buf())
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
        self.values.next()?.parse_unit()?;
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.values.next()?.parse_unit()?;
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
        visitor.visit_seq(super::Access::new(self.values, len))
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
        visitor.visit_seq(super::Access::new(self.values, len))
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
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(r#enum::Access::new(self.values))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(&self.values.next()?.parse_identifier()?)
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
    use crate::de::{error, parse::Values, Error, Position};
    use claim::{assert_err_eq, assert_ok_eq};
    use serde::{de, de::Visitor, Deserialize};
    use serde_bytes::ByteBuf;
    use serde_derive::Deserialize;
    use std::fmt;

    #[test]
    fn i8_valid() {
        let mut values = Values::new(b"42", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(i8::deserialize(deserializer), 42);
    }

    #[test]
    fn i8_invalid() {
        let mut values = Values::new(b"invalid", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            i8::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI8, Position::new(0, 0))
        );
    }

    #[test]
    fn i8_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42:100", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(i8::deserialize(deserializer), 42);
    }

    #[test]
    fn i16_valid() {
        let mut values = Values::new(b"42", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(i16::deserialize(deserializer), 42);
    }

    #[test]
    fn i16_invalid() {
        let mut values = Values::new(b"invalid", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            i16::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI16, Position::new(0, 0))
        );
    }

    #[test]
    fn i16_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42:100", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(i16::deserialize(deserializer), 42);
    }

    #[test]
    fn i32_valid() {
        let mut values = Values::new(b"42", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(i32::deserialize(deserializer), 42);
    }

    #[test]
    fn i32_invalid() {
        let mut values = Values::new(b"invalid", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            i32::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI32, Position::new(0, 0))
        );
    }

    #[test]
    fn i32_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42:100", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(i32::deserialize(deserializer), 42);
    }

    #[test]
    fn i64_valid() {
        let mut values = Values::new(b"42", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(i64::deserialize(deserializer), 42);
    }

    #[test]
    fn i64_invalid() {
        let mut values = Values::new(b"invalid", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            i64::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI64, Position::new(0, 0))
        );
    }

    #[test]
    fn i64_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42:100", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(i64::deserialize(deserializer), 42);
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128_valid() {
        let mut values = Values::new(b"42", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(i128::deserialize(deserializer), 42);
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128_invalid() {
        let mut values = Values::new(b"invalid", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            i128::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI128, Position::new(0, 0))
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42:100", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(i128::deserialize(deserializer), 42);
    }

    #[test]
    fn u8_valid() {
        let mut values = Values::new(b"42", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(u8::deserialize(deserializer), 42);
    }

    #[test]
    fn u8_invalid() {
        let mut values = Values::new(b"invalid", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            u8::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU8, Position::new(0, 0))
        );
    }

    #[test]
    fn u8_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42:100", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(u8::deserialize(deserializer), 42);
    }

    #[test]
    fn u16_valid() {
        let mut values = Values::new(b"42", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(u16::deserialize(deserializer), 42);
    }

    #[test]
    fn u16_invalid() {
        let mut values = Values::new(b"invalid", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            u16::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU16, Position::new(0, 0))
        );
    }

    #[test]
    fn u16_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42:100", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(u16::deserialize(deserializer), 42);
    }

    #[test]
    fn u32_valid() {
        let mut values = Values::new(b"42", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(u32::deserialize(deserializer), 42);
    }

    #[test]
    fn u32_invalid() {
        let mut values = Values::new(b"invalid", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            u32::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU32, Position::new(0, 0))
        );
    }

    #[test]
    fn u32_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42:100", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(u32::deserialize(deserializer), 42);
    }

    #[test]
    fn u64_valid() {
        let mut values = Values::new(b"42", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(u64::deserialize(deserializer), 42);
    }

    #[test]
    fn u64_invalid() {
        let mut values = Values::new(b"invalid", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            u64::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU64, Position::new(0, 0))
        );
    }

    #[test]
    fn u64_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42:100", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(u64::deserialize(deserializer), 42);
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128_valid() {
        let mut values = Values::new(b"42", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(u128::deserialize(deserializer), 42);
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128_invalid() {
        let mut values = Values::new(b"invalid", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            u128::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU128, Position::new(0, 0))
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42:100", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(u128::deserialize(deserializer), 42);
    }

    #[test]
    fn f32_valid() {
        let mut values = Values::new(b"42.9", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(f32::deserialize(deserializer), 42.9);
    }

    #[test]
    fn f32_invalid() {
        let mut values = Values::new(b"invalid", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            f32::deserialize(deserializer),
            Error::new(error::Kind::ExpectedF32, Position::new(0, 0))
        );
    }

    #[test]
    fn f32_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42.9:100.1", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(f32::deserialize(deserializer), 42.9);
    }

    #[test]
    fn f64_valid() {
        let mut values = Values::new(b"42.9", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(f64::deserialize(deserializer), 42.9);
    }

    #[test]
    fn f64_invalid() {
        let mut values = Values::new(b"invalid", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            f64::deserialize(deserializer),
            Error::new(error::Kind::ExpectedF64, Position::new(0, 0))
        );
    }

    #[test]
    fn f64_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42.9:100.1", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(f64::deserialize(deserializer), 42.9);
    }

    #[test]
    fn char_valid() {
        let mut values = Values::new(b"a", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(char::deserialize(deserializer), 'a');
    }

    #[test]
    fn char_invalid() {
        let mut values = Values::new(b"invalid", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            char::deserialize(deserializer),
            Error::new(error::Kind::ExpectedChar, Position::new(0, 0))
        );
    }

    #[test]
    fn char_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"a:b", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(char::deserialize(deserializer), 'a');
    }

    #[test]
    fn string_valid() {
        let mut values = Values::new(b"foo", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(String::deserialize(deserializer), "foo");
    }

    #[test]
    fn string_invalid() {
        let mut values = Values::new(b"\xF0\x9Ffoo", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            String::deserialize(deserializer),
            Error::new(error::Kind::ExpectedString, Position::new(0, 0))
        );
    }

    #[test]
    fn string_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"foo:bar", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(String::deserialize(deserializer), "foo");
    }

    #[test]
    fn byte_buf() {
        let mut values = Values::new(b"foo", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(ByteBuf::deserialize(deserializer), b"foo");
    }

    #[test]
    fn byte_buf_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"foo:bar", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(ByteBuf::deserialize(deserializer), b"foo");
    }

    #[test]
    fn unit() {
        let mut values = Values::new(b"", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(<()>::deserialize(deserializer), ());
    }

    #[test]
    fn unit_invalid() {
        let mut values = Values::new(b"invalid", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            <()>::deserialize(deserializer),
            Error::new(error::Kind::ExpectedUnit, Position::new(0, 0))
        );
    }

    #[test]
    fn unit_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b":", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(<()>::deserialize(deserializer), ());
    }

    #[test]
    fn unit_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Unit;
        let mut values = Values::new(b"", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(Unit::deserialize(deserializer), Unit);
    }

    #[test]
    fn unit_struct_invalid() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Unit;
        let mut values = Values::new(b"invalid", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            Unit::deserialize(deserializer),
            Error::new(error::Kind::ExpectedUnit, Position::new(0, 0))
        );
    }

    #[test]
    fn unit_struct_multiple_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Unit;
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b":", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(Unit::deserialize(deserializer), Unit);
    }

    #[test]
    fn newtype_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Newtype(u64);
        let mut values = Values::new(b"42", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(Newtype::deserialize(deserializer), Newtype(42));
    }

    #[test]
    fn newtype_struct_multiple_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Newtype(u64);
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42:100", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(Newtype::deserialize(deserializer), Newtype(42));
    }

    #[test]
    fn tuple() {
        let mut values = Values::new(b"42:foo::1.2", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(
            <(u64, String, (), f64)>::deserialize(deserializer),
            (42, "foo".to_owned(), (), 1.2)
        );
    }

    #[test]
    fn tuple_trailing_values() {
        // The entire values iterator is not consumed. Just the requested tuple values are
        // consumed.
        let mut values = Values::new(b"42:foo::1.2:not:consumed", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(
            <(u64, String, (), f64)>::deserialize(deserializer),
            (42, "foo".to_owned(), (), 1.2)
        );
    }

    #[test]
    fn tuple_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TupleStruct(u64, String, (), f64);
        let mut values = Values::new(b"42:foo::1.2", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(
            TupleStruct::deserialize(deserializer),
            TupleStruct(42, "foo".to_owned(), (), 1.2)
        );
    }

    #[test]
    fn tuple_struct_trailing_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TupleStruct(u64, String, (), f64);
        // The entire values iterator is not consumed. Just the requested tuple values are
        // consumed.
        let mut values = Values::new(b"42:foo::1.2:not:consumed", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(
            TupleStruct::deserialize(deserializer),
            TupleStruct(42, "foo".to_owned(), (), 1.2)
        );
    }

    #[test]
    fn enum_unit_variant() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Unit {
            Variant,
        }
        let mut values = Values::new(b"Variant", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(Unit::deserialize(deserializer), Unit::Variant,);
    }

    #[test]
    fn enum_unit_variant_trailing_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Unit {
            Variant,
        }
        let mut values = Values::new(b"Variant:42", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(Unit::deserialize(deserializer), Unit::Variant,);
    }

    #[test]
    fn enum_newtype_variant() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Newtype {
            Variant(u64),
        }
        let mut values = Values::new(b"Variant:42", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(Newtype::deserialize(deserializer), Newtype::Variant(42),);
    }

    #[test]
    fn enum_newtype_variant_trailing_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Newtype {
            Variant(u64),
        }
        let mut values = Values::new(b"Variant:42:foo", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(Newtype::deserialize(deserializer), Newtype::Variant(42),);
    }

    #[test]
    fn enum_tuple_variant() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Tuple {
            Variant(u64, String),
        }
        let mut values = Values::new(b"Variant:42:foo", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(
            Tuple::deserialize(deserializer),
            Tuple::Variant(42, "foo".to_owned()),
        );
    }

    #[test]
    fn enum_tuple_variant_trailing_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Tuple {
            Variant(u64, String),
        }
        let mut values = Values::new(b"Variant:42:foo:bar", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(
            Tuple::deserialize(deserializer),
            Tuple::Variant(42, "foo".to_owned()),
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
        let mut values = Values::new(b"foo", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(
            Identifier::deserialize(deserializer),
            Identifier("foo".to_owned()),
        );
    }

    #[test]
    fn identifier_trailing_values() {
        let mut values = Values::new(b"foo:bar", 0, 0);
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(
            Identifier::deserialize(deserializer),
            Identifier("foo".to_owned()),
        );
    }
}
