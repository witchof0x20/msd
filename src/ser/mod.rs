mod error;
mod r#struct;
mod write;

pub use error::{Error, Result};

use serde::{ser, ser::Impossible, Serialize};
use std::{fmt, fmt::Display, io::Write};
use write::WriteExt;

pub struct Serializer<W> {
    writer: W,
}

impl<W> Serializer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = r#struct::Serializer<'a, W>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    #[cfg(has_i128)]
    fn serialize_i128(self, v: i128) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    #[cfg(has_u128)]
    fn serialize_u128(self, v: u128) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_some<T>(self, v: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnsupportedType)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        Err(Error::UnsupportedType)
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnsupportedType)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnsupportedType)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::UnsupportedType)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::UnsupportedType)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::UnsupportedType)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::UnsupportedType)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::UnsupportedType)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(r#struct::Serializer::new(&mut self.writer))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::UnsupportedType)
    }
}

#[cfg(test)]
mod tests {
    use super::Serializer;
    use claim::assert_ok;
    use serde::Serialize;

    #[test]
    fn single_field() {
        #[derive(serde_derive::Serialize)]
        struct Foo {
            bar: usize,
        }
        let mut output = Vec::new();

        assert_ok!(Foo { bar: 42 }.serialize(&mut Serializer::new(&mut output)));
        assert_eq!(output, b"#bar:42;\n");
    }

    #[test]
    fn multiple_fields() {
        #[derive(serde_derive::Serialize)]
        struct Foo {
            bar: usize,
            baz: Option<()>,
            qux: Option<&'static str>,
        }
        let mut output = Vec::new();

        assert_ok!(Foo {
            bar: 42,
            baz: None,
            qux: Some("test:test"),
        }
        .serialize(&mut Serializer::new(&mut output)));
        assert_eq!(output, b"#bar:42;\n#qux:test\\:test;\n");
    }
}
