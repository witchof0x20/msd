use crate::de::{error, Error, Position, Result};
use serde::{de, de::Visitor};

pub(in super::super) struct Deserializer<'a> {
    identifier: &'a str,
    position: Position,
}

impl<'a> Deserializer<'a> {
    pub(in super::super) fn new(identifier: &'a str, position: Position) -> Self {
        Self {
            identifier,
            position,
        }
    }
}

impl<'a, 'de> de::Deserializer<'de> for Deserializer<'a> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::CannotDeserializeAsSelfDescribing,
            self.position,
        ))
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    #[cfg(has_i128)]
    fn deserialize_i128<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    #[cfg(has_i128)]
    fn deserialize_u128<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
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
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
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
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
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
        Err(Error::new(
            error::Kind::MustDeserializeStructFieldAsIdentifier,
            self.position,
        ))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor
            .visit_str(self.identifier)
            .map_err(|mut error: Error| {
                error.set_position(self.position);
                error
            })
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::new(
            error::Kind::CannotDeserializeAsSelfDescribing,
            self.position,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::Deserializer;
    use crate::de::{error, Error, Position};
    use claims::{assert_err_eq, assert_ok_eq};
    use serde::{de, de::Visitor, Deserialize};
    use serde_bytes::{ByteBuf, Bytes};
    use serde_derive::Deserialize;
    use std::{collections::HashMap, fmt};

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
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_ok_eq!(
            Identifier::deserialize(deserializer),
            Identifier("foo".to_owned())
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

        let deserializer = Deserializer::new("foo", Position::new(1, 2));

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

        let deserializer = Deserializer::new("foo", Position::new(1, 2));

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

        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            IgnoredAny::deserialize(deserializer),
            Error::new(
                error::Kind::CannotDeserializeAsSelfDescribing,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn bool() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            bool::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn i8() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            i8::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn i16() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            i16::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn i32() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            i32::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn i64() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            i64::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn i128() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            i128::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn u8() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            u8::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn u16() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            u16::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn u32() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            u32::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn u64() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            u64::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn u128() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            u128::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn f32() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            f32::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn f64() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            f64::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn char() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            char::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn str() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            <&str>::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn string() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            String::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn bytes() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            <&Bytes>::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn byte_buf() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            ByteBuf::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn option() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            Option::<()>::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn unit() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            <()>::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn unit_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Unit;

        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            Unit::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn newtype_struct() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Newtype(u64);

        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            Newtype::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn seq() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            Vec::<()>::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn tuple() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            <((),)>::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn tuple_struct() {
        #[derive(Debug, Deserialize)]
        struct TupleStruct(String, u64, (), f64);

        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            TupleStruct::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn map() {
        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            HashMap::<(), ()>::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn r#struct() {
        #[derive(Debug, Deserialize)]
        struct Struct {
            _foo: String,
            _bar: u64,
            _baz: (),
            _qux: f64,
        }

        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            Struct::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }

    #[test]
    fn r#enum() {
        #[derive(Debug, Deserialize)]
        enum Enum {}

        let deserializer = Deserializer::new("foo", Position::new(1, 2));

        assert_err_eq!(
            Enum::deserialize(deserializer),
            Error::new(
                error::Kind::MustDeserializeStructFieldAsIdentifier,
                Position::new(1, 2)
            )
        );
    }
}
