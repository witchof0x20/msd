use crate::ser::{tuple, Error, Result, WriteExt};
use serde::{ser, ser::Impossible, Serialize};
use std::io::Write;

pub(in super::super) struct Serializer<'a, W> {
    writer: &'a mut W,
}

impl<'a, W> Serializer<'a, W> {
    pub(in super::super) fn new(writer: &'a mut W) -> Self {
        Self { writer }
    }
}

impl<'a, W> ser::Serializer for Serializer<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = tuple::tag::nested::Serializer<'a, W>;
    type SerializeTupleStruct = tuple::tag::nested::Serializer<'a, W>;
    type SerializeTupleVariant = tuple::nested::Serializer<'a, W>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        if v {
            self.writer.write_tag_name_unescaped(b"true")
        } else {
            self.writer.write_tag_name_unescaped(b"false")
        }
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_tag_name_unescaped(s.as_bytes())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_tag_name_unescaped(s.as_bytes())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_tag_name_unescaped(s.as_bytes())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_tag_name_unescaped(s.as_bytes())
    }

    #[cfg(has_i128)]
    fn serialize_i128(self, v: i128) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_tag_name_unescaped(s.as_bytes())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_tag_name_unescaped(s.as_bytes())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_tag_name_unescaped(s.as_bytes())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_tag_name_unescaped(s.as_bytes())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_tag_name_unescaped(s.as_bytes())
    }

    #[cfg(has_u128)]
    fn serialize_u128(self, v: u128) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_tag_name_unescaped(s.as_bytes())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        let mut buffer = ryu::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_tag_name_unescaped(s.as_bytes())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        let mut buffer = ryu::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_tag_name_unescaped(s.as_bytes())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        let mut buffer = [0; 4];
        v.encode_utf8(&mut buffer);
        self.writer.write_tag_name_escaped(&buffer[..v.len_utf8()])
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(v)
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        self.writer.write_tag_name_unescaped(b"")
    }

    fn serialize_some<T>(self, v: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        v.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        self.writer.write_tag_name_unescaped(b"")
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.writer.write_tag_name_unescaped(b"")
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(variant.as_bytes())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        self.writer.write_tag_name_escaped(variant.as_bytes())?;
        value.serialize(tuple::element::Serializer::new(self.writer))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::UnsupportedType)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(tuple::tag::nested::Serializer::new(self.writer))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(tuple::tag::nested::Serializer::new(self.writer))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.writer.write_tag_name_escaped(variant.as_bytes())?;
        Ok(tuple::nested::Serializer::new(self.writer))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::UnsupportedType)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::UnsupportedType)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::UnsupportedType)
    }
}

#[cfg(test)]
mod tests {
    use super::Serializer;
    use claim::assert_ok;
    use serde::{
        ser::{SerializeTupleStruct, SerializeTupleVariant},
        Serialize,
    };
    use serde_bytes::Bytes;
    use serde_derive::Serialize;

    #[test]
    fn r#true() {
        let mut output = Vec::new();

        assert_ok!(true.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#true");
    }

    #[test]
    fn r#false() {
        let mut output = Vec::new();

        assert_ok!(false.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#false");
    }

    #[test]
    fn i8() {
        let mut output = Vec::new();

        assert_ok!(42i8.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#42");
    }

    #[test]
    fn i16() {
        let mut output = Vec::new();

        assert_ok!(42i16.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#42");
    }

    #[test]
    fn i32() {
        let mut output = Vec::new();

        assert_ok!(42i32.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#42");
    }

    #[test]
    fn i64() {
        let mut output = Vec::new();

        assert_ok!(42i64.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#42");
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128() {
        let mut output = Vec::new();

        assert_ok!(42i128.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#42");
    }

    #[test]
    fn i8_neg() {
        let mut output = Vec::new();

        assert_ok!((-42i8).serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#-42");
    }

    #[test]
    fn i16_neg() {
        let mut output = Vec::new();

        assert_ok!((-42i16).serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#-42");
    }

    #[test]
    fn i32_neg() {
        let mut output = Vec::new();

        assert_ok!((-42i32).serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#-42");
    }

    #[test]
    fn i64_neg() {
        let mut output = Vec::new();

        assert_ok!((-42i64).serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#-42");
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128_neg() {
        let mut output = Vec::new();

        assert_ok!((-42i128).serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#-42");
    }

    #[test]
    fn u8() {
        let mut output = Vec::new();

        assert_ok!(42u8.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#42");
    }

    #[test]
    fn u16() {
        let mut output = Vec::new();

        assert_ok!(42u16.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#42");
    }

    #[test]
    fn u32() {
        let mut output = Vec::new();

        assert_ok!(42u32.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#42");
    }

    #[test]
    fn u64() {
        let mut output = Vec::new();

        assert_ok!(42u64.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#42");
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128() {
        let mut output = Vec::new();

        assert_ok!(42u128.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#42");
    }

    #[test]
    fn f32() {
        let mut output = Vec::new();

        assert_ok!(42f32.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#42.0");
    }

    #[test]
    fn f64() {
        let mut output = Vec::new();

        assert_ok!(42f64.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#42.0");
    }

    #[test]
    fn char() {
        let mut output = Vec::new();

        assert_ok!('a'.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#a");
    }

    #[test]
    fn char_escape_number_sign() {
        let mut output = Vec::new();

        assert_ok!('#'.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#\\#");
    }

    #[test]
    fn char_escape_colon() {
        let mut output = Vec::new();

        assert_ok!(':'.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#\\:");
    }

    #[test]
    fn char_escape_semicolon() {
        let mut output = Vec::new();

        assert_ok!(';'.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#\\;");
    }

    #[test]
    fn char_escape_backslash() {
        let mut output = Vec::new();

        assert_ok!('\\'.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#\\\\");
    }

    #[test]
    fn char_does_not_escape_forward_slash() {
        let mut output = Vec::new();

        assert_ok!('/'.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#/");
    }

    #[test]
    fn str() {
        let mut output = Vec::new();

        assert_ok!("bar".serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#bar");
    }

    #[test]
    fn str_escape_number_sign() {
        let mut output = Vec::new();

        assert_ok!("ba#r".serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#ba\\#r");
    }

    #[test]
    fn str_escape_colon() {
        let mut output = Vec::new();

        assert_ok!("ba:r".serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#ba\\:r");
    }

    #[test]
    fn str_escape_semicolon() {
        let mut output = Vec::new();

        assert_ok!("ba;r".serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#ba\\;r");
    }

    #[test]
    fn str_escape_backslash() {
        let mut output = Vec::new();

        assert_ok!("ba\\r".serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#ba\\\\r");
    }

    #[test]
    fn str_escape_double_forwardslash() {
        let mut output = Vec::new();

        assert_ok!("ba//r".serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#ba\\/\\/r");
    }

    #[test]
    fn str_do_not_escape_single_forwardslash() {
        let mut output = Vec::new();

        assert_ok!("ba/r".serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#ba/r");
    }

    #[test]
    fn bytes() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"bar").serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#bar");
    }

    #[test]
    fn bytes_escape_number_sign() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"ba#r").serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#ba\\#r");
    }

    #[test]
    fn bytes_escape_colon() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"ba:r").serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#ba\\:r");
    }

    #[test]
    fn bytes_escape_semicolon() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"ba;r").serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#ba\\;r");
    }

    #[test]
    fn bytes_escape_backslash() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"ba\\r").serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#ba\\\\r");
    }

    #[test]
    fn bytes_escape_double_forwardslash() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"ba//r").serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#ba\\/\\/r");
    }

    #[test]
    fn bytes_do_not_escape_single_forwardslash() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"ba/r").serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#ba/r");
    }

    #[test]
    fn none() {
        let mut output = Vec::new();

        assert_ok!(Option::<()>::None.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#");
    }

    #[test]
    fn some() {
        let mut output = Vec::new();

        assert_ok!(Some(42).serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#42");
    }

    #[test]
    fn unit() {
        let mut output = Vec::new();

        assert_ok!(().serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#");
    }

    #[test]
    fn unit_struct() {
        #[derive(Serialize)]
        struct Bar;

        let mut output = Vec::new();

        assert_ok!(Bar.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#");
    }

    #[test]
    fn unit_variant() {
        #[derive(Serialize)]
        enum Enum {
            A,
        }

        let mut output = Vec::new();

        assert_ok!(Enum::A.serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#A");
    }

    #[test]
    fn newtype_struct() {
        #[derive(Serialize)]
        struct NewtypeStruct(u32);

        let mut output = Vec::new();

        assert_ok!(NewtypeStruct(42).serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#42");
    }

    #[test]
    fn newtype_variant() {
        #[derive(Serialize)]
        enum Newtype {
            Variant(u32),
        }

        let mut output = Vec::new();

        assert_ok!(Newtype::Variant(42).serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#Variant:42");
    }

    #[test]
    fn empty_tuple() {
        let mut output = Vec::new();

        assert_ok!(<[(); 0]>::serialize(&[], Serializer::new(&mut output)));

        assert_eq!(output, b"#");
    }

    #[test]
    fn single_element_tuple() {
        let mut output = Vec::new();

        assert_ok!((42).serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#42");
    }

    #[test]
    fn multiple_element_tuple() {
        let mut output = Vec::new();

        assert_ok!((42, "bar", (), 1.0).serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#42:bar::1.0");
    }

    #[test]
    fn empty_tuple_struct() {
        #[derive(Serialize)]
        struct TupleStruct();

        let mut output = Vec::new();

        assert_ok!(TupleStruct().serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#");
    }

    #[test]
    fn single_element_tuple_struct() {
        struct TupleStruct(usize);
        impl Serialize for TupleStruct {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut ts = serializer.serialize_tuple_struct("TupleStruct", 1)?;
                ts.serialize_field(&self.0)?;
                ts.end()
            }
        }

        let mut output = Vec::new();

        assert_ok!(TupleStruct(42).serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#42");
    }

    #[test]
    fn multiple_element_tuple_struct() {
        #[derive(Serialize)]
        struct TupleStruct(usize, &'static str, (), f32);

        let mut output = Vec::new();

        assert_ok!(TupleStruct(42, "bar", (), 1.0).serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#42:bar::1.0");
    }

    #[test]
    fn empty_tuple_variant() {
        enum TupleEnum {
            Variant(),
        }
        impl Serialize for TupleEnum {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer
                    .serialize_tuple_variant("TupleEnum", 0, "Variant", 0)?
                    .end()
            }
        }

        let mut output = Vec::new();

        assert_ok!(TupleEnum::Variant().serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#Variant");
    }

    #[test]
    fn single_element_tuple_variant() {
        enum TupleEnum {
            Variant(usize),
        }
        impl Serialize for TupleEnum {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let Self::Variant(inner) = self;
                let mut tv = serializer.serialize_tuple_variant("TupleEnum", 0, "Variant", 1)?;
                tv.serialize_field(&inner)?;
                tv.end()
            }
        }

        let mut output = Vec::new();

        assert_ok!(TupleEnum::Variant(42).serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#Variant:42");
    }

    #[test]
    fn multiple_element_tuple_variant() {
        #[derive(Serialize)]
        enum TupleEnum {
            Variant(usize, &'static str, (), f32),
        }

        let mut output = Vec::new();

        assert_ok!(
            TupleEnum::Variant(42, "bar", (), 1.0).serialize(Serializer::new(&mut output))
        );

        assert_eq!(output, b"#Variant:42:bar::1.0");
    }

    #[test]
    fn nested_tuple_variant() {
        #[derive(Serialize)]
        enum TupleEnum {
            Variant(usize, (usize, usize), ((usize, usize), usize), usize),
        }

        let mut output = Vec::new();

        assert_ok!(TupleEnum::Variant(1, (2, 3), ((4, 5), 6), 7)
            .serialize(Serializer::new(&mut output)));

        assert_eq!(output, b"#Variant:1:2:3:4:5:6:7");
    }
}
