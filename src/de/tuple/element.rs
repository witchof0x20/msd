use crate::de::{error, parse::Values, r#enum, Error, Result};
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
        Err(Error::new(
            error::Kind::CannotDeserializeAsSelfDescribing,
            self.values.current_position(),
        ))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        visitor
            .visit_bool(value.parse_bool()?)
            .map_err(|mut error: Error| {
                error.set_position(value.position());
                error
            })
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        visitor
            .visit_i8(value.parse_i8()?)
            .map_err(|mut error: Error| {
                error.set_position(value.position());
                error
            })
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        visitor
            .visit_i16(value.parse_i16()?)
            .map_err(|mut error: Error| {
                error.set_position(value.position());
                error
            })
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        visitor
            .visit_i32(value.parse_i32()?)
            .map_err(|mut error: Error| {
                error.set_position(value.position());
                error
            })
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        visitor
            .visit_i64(value.parse_i64()?)
            .map_err(|mut error: Error| {
                error.set_position(value.position());
                error
            })
    }

    #[cfg(has_i128)]
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        visitor
            .visit_i128(value.parse_i128()?)
            .map_err(|mut error: Error| {
                error.set_position(value.position());
                error
            })
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        visitor
            .visit_u8(value.parse_u8()?)
            .map_err(|mut error: Error| {
                error.set_position(value.position());
                error
            })
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        visitor
            .visit_u16(value.parse_u16()?)
            .map_err(|mut error: Error| {
                error.set_position(value.position());
                error
            })
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        visitor
            .visit_u32(value.parse_u32()?)
            .map_err(|mut error: Error| {
                error.set_position(value.position());
                error
            })
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        visitor
            .visit_u64(value.parse_u64()?)
            .map_err(|mut error: Error| {
                error.set_position(value.position());
                error
            })
    }

    #[cfg(has_i128)]
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        visitor
            .visit_u128(value.parse_u128()?)
            .map_err(|mut error: Error| {
                error.set_position(value.position());
                error
            })
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        visitor
            .visit_f32(value.parse_f32()?)
            .map_err(|mut error: Error| {
                error.set_position(value.position());
                error
            })
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        visitor
            .visit_f64(value.parse_f64()?)
            .map_err(|mut error: Error| {
                error.set_position(value.position());
                error
            })
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        visitor
            .visit_char(value.parse_char()?)
            .map_err(|mut error: Error| {
                error.set_position(value.position());
                error
            })
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        visitor
            .visit_str(&value.parse_string()?)
            .map_err(|mut error: Error| {
                error.set_position(value.position());
                error
            })
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        visitor
            .visit_string(value.parse_string()?)
            .map_err(|mut error: Error| {
                error.set_position(value.position());
                error
            })
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        visitor
            .visit_bytes(&value.parse_byte_buf())
            .map_err(|mut error: Error| {
                error.set_position(value.position());
                error
            })
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        visitor
            .visit_byte_buf(value.parse_byte_buf())
            .map_err(|mut error: Error| {
                error.set_position(value.position());
                error
            })
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::CannotDeserializeAsOptionInTuple,
            self.values.current_position(),
        ))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        value.parse_unit()?;
        visitor.visit_unit().map_err(|mut error: Error| {
            error.set_position(value.position());
            error
        })
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.values.next()?;
        value.parse_unit()?;
        visitor.visit_unit().map_err(|mut error: Error| {
            error.set_position(value.position());
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
        Err(Error::new(
            error::Kind::CannotDeserializeAsSeqInTuple,
            self.values.current_position(),
        ))
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
        Err(Error::new(
            error::Kind::CannotDeserializeAsMapInTuple,
            self.values.current_position(),
        ))
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
        Err(Error::new(
            error::Kind::CannotDeserializeAsStructInTuple,
            self.values.current_position(),
        ))
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
        let value = self.values.next()?;
        visitor
            .visit_str(&value.parse_identifier()?)
            .map_err(|mut error: Error| {
                error.set_position(value.position());
                error
            })
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::CannotDeserializeAsSelfDescribing,
            self.values.current_position(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::Deserializer;
    use crate::de::{error, parse::Values, Error, Position};
    use claims::{assert_err_eq, assert_ok_eq};
    use serde::{de, de::Visitor, Deserialize};
    use serde_bytes::ByteBuf;
    use serde_derive::Deserialize;
    use std::{collections::HashMap, fmt};

    #[test]
    fn bool_true() {
        let mut values = Values::new(b"true", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(bool::deserialize(deserializer), true);
    }

    #[test]
    fn bool_false() {
        let mut values = Values::new(b"false", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(bool::deserialize(deserializer), false);
    }

    #[test]
    fn bool_invalid() {
        let mut values = Values::new(b"invalid", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            bool::deserialize(deserializer),
            Error::new(error::Kind::ExpectedBool, Position::new(0, 0))
        );
    }

    #[test]
    fn bool_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"true:false", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(bool::deserialize(deserializer), true);
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

        let mut values = Values::new(b"true", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomBool::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
        );
    }

    #[test]
    fn i8_valid() {
        let mut values = Values::new(b"42", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(i8::deserialize(deserializer), 42);
    }

    #[test]
    fn i8_invalid() {
        let mut values = Values::new(b"invalid", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            i8::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI8, Position::new(0, 0))
        );
    }

    #[test]
    fn i8_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42:100", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(i8::deserialize(deserializer), 42);
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

        let mut values = Values::new(b"42", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomI8::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
        );
    }

    #[test]
    fn i16_valid() {
        let mut values = Values::new(b"42", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(i16::deserialize(deserializer), 42);
    }

    #[test]
    fn i16_invalid() {
        let mut values = Values::new(b"invalid", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            i16::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI16, Position::new(0, 0))
        );
    }

    #[test]
    fn i16_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42:100", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(i16::deserialize(deserializer), 42);
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

        let mut values = Values::new(b"42", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomI16::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
        );
    }

    #[test]
    fn i32_valid() {
        let mut values = Values::new(b"42", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(i32::deserialize(deserializer), 42);
    }

    #[test]
    fn i32_invalid() {
        let mut values = Values::new(b"invalid", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            i32::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI32, Position::new(0, 0))
        );
    }

    #[test]
    fn i32_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42:100", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(i32::deserialize(deserializer), 42);
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

        let mut values = Values::new(b"42", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomI32::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
        );
    }

    #[test]
    fn i64_valid() {
        let mut values = Values::new(b"42", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(i64::deserialize(deserializer), 42);
    }

    #[test]
    fn i64_invalid() {
        let mut values = Values::new(b"invalid", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            i64::deserialize(deserializer),
            Error::new(error::Kind::ExpectedI64, Position::new(0, 0))
        );
    }

    #[test]
    fn i64_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42:100", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(i64::deserialize(deserializer), 42);
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

        let mut values = Values::new(b"42", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomI64::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128_valid() {
        let mut values = Values::new(b"42", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(i128::deserialize(deserializer), 42);
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128_invalid() {
        let mut values = Values::new(b"invalid", Position::new(0, 0));
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
        let mut values = Values::new(b"42:100", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(i128::deserialize(deserializer), 42);
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

        let mut values = Values::new(b"42", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomI128::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
        );
    }

    #[test]
    fn u8_valid() {
        let mut values = Values::new(b"42", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(u8::deserialize(deserializer), 42);
    }

    #[test]
    fn u8_invalid() {
        let mut values = Values::new(b"invalid", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            u8::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU8, Position::new(0, 0))
        );
    }

    #[test]
    fn u8_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42:100", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(u8::deserialize(deserializer), 42);
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

        let mut values = Values::new(b"42", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomU8::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
        );
    }

    #[test]
    fn u16_valid() {
        let mut values = Values::new(b"42", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(u16::deserialize(deserializer), 42);
    }

    #[test]
    fn u16_invalid() {
        let mut values = Values::new(b"invalid", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            u16::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU16, Position::new(0, 0))
        );
    }

    #[test]
    fn u16_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42:100", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(u16::deserialize(deserializer), 42);
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

        let mut values = Values::new(b"42", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomU16::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
        );
    }

    #[test]
    fn u32_valid() {
        let mut values = Values::new(b"42", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(u32::deserialize(deserializer), 42);
    }

    #[test]
    fn u32_invalid() {
        let mut values = Values::new(b"invalid", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            u32::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU32, Position::new(0, 0))
        );
    }

    #[test]
    fn u32_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42:100", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(u32::deserialize(deserializer), 42);
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

        let mut values = Values::new(b"42", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomU32::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
        );
    }

    #[test]
    fn u64_valid() {
        let mut values = Values::new(b"42", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(u64::deserialize(deserializer), 42);
    }

    #[test]
    fn u64_invalid() {
        let mut values = Values::new(b"invalid", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            u64::deserialize(deserializer),
            Error::new(error::Kind::ExpectedU64, Position::new(0, 0))
        );
    }

    #[test]
    fn u64_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42:100", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(u64::deserialize(deserializer), 42);
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

        let mut values = Values::new(b"42", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomU64::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
        );
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128_valid() {
        let mut values = Values::new(b"42", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(u128::deserialize(deserializer), 42);
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128_invalid() {
        let mut values = Values::new(b"invalid", Position::new(0, 0));
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
        let mut values = Values::new(b"42:100", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(u128::deserialize(deserializer), 42);
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

        let mut values = Values::new(b"42", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomU128::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
        );
    }

    #[test]
    fn f32_valid() {
        let mut values = Values::new(b"42.9", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(f32::deserialize(deserializer), 42.9);
    }

    #[test]
    fn f32_invalid() {
        let mut values = Values::new(b"invalid", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            f32::deserialize(deserializer),
            Error::new(error::Kind::ExpectedF32, Position::new(0, 0))
        );
    }

    #[test]
    fn f32_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42.9:100.1", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(f32::deserialize(deserializer), 42.9);
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

        let mut values = Values::new(b"42", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomF32::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
        );
    }

    #[test]
    fn f64_valid() {
        let mut values = Values::new(b"42.9", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(f64::deserialize(deserializer), 42.9);
    }

    #[test]
    fn f64_invalid() {
        let mut values = Values::new(b"invalid", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            f64::deserialize(deserializer),
            Error::new(error::Kind::ExpectedF64, Position::new(0, 0))
        );
    }

    #[test]
    fn f64_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42.9:100.1", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(f64::deserialize(deserializer), 42.9);
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

        let mut values = Values::new(b"42", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomF64::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
        );
    }

    #[test]
    fn char_valid() {
        let mut values = Values::new(b"a", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(char::deserialize(deserializer), 'a');
    }

    #[test]
    fn char_invalid() {
        let mut values = Values::new(b"invalid", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            char::deserialize(deserializer),
            Error::new(error::Kind::ExpectedChar, Position::new(0, 0))
        );
    }

    #[test]
    fn char_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"a:b", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(char::deserialize(deserializer), 'a');
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

        let mut values = Values::new(b"a", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomChar::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
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
    fn str_valid() {
        let mut values = Values::new(b"foo", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(Str::deserialize(deserializer), Str("foo".to_owned()));
    }

    #[test]
    fn str_invalid() {
        let mut values = Values::new(b"\xF0\x9Ffoo", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            Str::deserialize(deserializer),
            Error::new(error::Kind::ExpectedString, Position::new(0, 0))
        );
    }

    #[test]
    fn str_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"foo:bar", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(Str::deserialize(deserializer), Str("foo".to_owned()));
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

        let mut values = Values::new(b"a", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomStr::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
        );
    }

    #[test]
    fn string_valid() {
        let mut values = Values::new(b"foo", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(String::deserialize(deserializer), "foo");
    }

    #[test]
    fn string_invalid() {
        let mut values = Values::new(b"\xF0\x9Ffoo", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            String::deserialize(deserializer),
            Error::new(error::Kind::ExpectedString, Position::new(0, 0))
        );
    }

    #[test]
    fn string_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"foo:bar", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(String::deserialize(deserializer), "foo");
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

        let mut values = Values::new(b"a", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomString::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
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
        let mut values = Values::new(b"foo", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(Bytes::deserialize(deserializer), Bytes(b"foo".to_vec()));
    }

    #[test]
    fn bytes_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"foo:bar", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(Bytes::deserialize(deserializer), Bytes(b"foo".to_vec()));
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

        let mut values = Values::new(b"a", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomBytes::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
        );
    }

    #[test]
    fn byte_buf() {
        let mut values = Values::new(b"foo", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(ByteBuf::deserialize(deserializer), b"foo");
    }

    #[test]
    fn byte_buf_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"foo:bar", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(ByteBuf::deserialize(deserializer), b"foo");
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

        let mut values = Values::new(b"a", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomByteBuf::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
        );
    }

    #[test]
    fn unit() {
        let mut values = Values::new(b"", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(<()>::deserialize(deserializer), ());
    }

    #[test]
    fn unit_invalid() {
        let mut values = Values::new(b"invalid", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            <()>::deserialize(deserializer),
            Error::new(error::Kind::ExpectedUnit, Position::new(0, 0))
        );
    }

    #[test]
    fn unit_multiple_values() {
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b":", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(<()>::deserialize(deserializer), ());
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

        let mut values = Values::new(b"", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomUnit::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
        );
    }

    #[test]
    fn unit_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Unit;
        let mut values = Values::new(b"", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(Unit::deserialize(deserializer), Unit);
    }

    #[test]
    fn unit_struct_invalid() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Unit;
        let mut values = Values::new(b"invalid", Position::new(0, 0));
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
        let mut values = Values::new(b":", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(Unit::deserialize(deserializer), Unit);
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

        let mut values = Values::new(b"", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomUnitStruct::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
        );
    }

    #[test]
    fn newtype_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Newtype(u64);
        let mut values = Values::new(b"42", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(Newtype::deserialize(deserializer), Newtype(42));
    }

    #[test]
    fn newtype_struct_multiple_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Newtype(u64);
        // The entire values iterator is not consumed. Just the first value is returned.
        let mut values = Values::new(b"42:100", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(Newtype::deserialize(deserializer), Newtype(42));
    }

    #[test]
    fn tuple() {
        let mut values = Values::new(b"42:foo::1.2", Position::new(0, 0));
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
        let mut values = Values::new(b"42:foo::1.2:not:consumed", Position::new(0, 0));
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
        let mut values = Values::new(b"42:foo::1.2", Position::new(0, 0));
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
        let mut values = Values::new(b"42:foo::1.2:not:consumed", Position::new(0, 0));
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
        let mut values = Values::new(b"Variant", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(Unit::deserialize(deserializer), Unit::Variant,);
    }

    #[test]
    fn enum_unit_variant_trailing_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Unit {
            Variant,
        }
        let mut values = Values::new(b"Variant:42", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(Unit::deserialize(deserializer), Unit::Variant,);
    }

    #[test]
    fn enum_newtype_variant() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Newtype {
            Variant(u64),
        }
        let mut values = Values::new(b"Variant:42", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(Newtype::deserialize(deserializer), Newtype::Variant(42),);
    }

    #[test]
    fn enum_newtype_variant_trailing_values() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Newtype {
            Variant(u64),
        }
        let mut values = Values::new(b"Variant:42:foo", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(Newtype::deserialize(deserializer), Newtype::Variant(42),);
    }

    #[test]
    fn enum_tuple_variant() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Tuple {
            Variant(u64, String),
        }
        let mut values = Values::new(b"Variant:42:foo", Position::new(0, 0));
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
        let mut values = Values::new(b"Variant:42:foo:bar", Position::new(0, 0));
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
        let mut values = Values::new(b"foo", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(
            Identifier::deserialize(deserializer),
            Identifier("foo".to_owned()),
        );
    }

    #[test]
    fn identifier_trailing_values() {
        let mut values = Values::new(b"foo:bar", Position::new(0, 0));
        let deserializer = Deserializer::new(&mut values);

        assert_ok_eq!(
            Identifier::deserialize(deserializer),
            Identifier("foo".to_owned()),
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

        let mut values = Values::new(b"a", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            CustomIdentifier::deserialize(deserializer),
            Error::new(error::Kind::Custom("foo".to_string()), Position::new(1, 2))
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

        let mut values = Values::new(b"", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            Any::deserialize(deserializer),
            Error::new(
                error::Kind::CannotDeserializeAsSelfDescribing,
                Position::new(1, 2)
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

        let mut values = Values::new(b"", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            IgnoredAny::deserialize(deserializer),
            Error::new(
                error::Kind::CannotDeserializeAsSelfDescribing,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn option() {
        let mut values = Values::new(b"", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            Option::<()>::deserialize(deserializer),
            Error::new(
                error::Kind::CannotDeserializeAsOptionInTuple,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn seq() {
        let mut values = Values::new(b"", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            Vec::<()>::deserialize(deserializer),
            Error::new(
                error::Kind::CannotDeserializeAsSeqInTuple,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn map() {
        let mut values = Values::new(b"", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            HashMap::<(), ()>::deserialize(deserializer),
            Error::new(
                error::Kind::CannotDeserializeAsMapInTuple,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn r#struct() {
        #[derive(Debug, Deserialize)]
        struct Struct {
            _foo: usize,
            _bar: bool,
        }

        let mut values = Values::new(b"", Position::new(1, 2));
        let deserializer = Deserializer::new(&mut values);

        assert_err_eq!(
            Struct::deserialize(deserializer),
            Error::new(
                error::Kind::CannotDeserializeAsStructInTuple,
                Position::new(1, 2)
            )
        );
    }
}
