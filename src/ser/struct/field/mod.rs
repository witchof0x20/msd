mod parameter;
mod tuple;

use crate::{escape::Escaper, ser::{Error, Result, WriteExt}};
use serde::{ser, ser::Impossible, Serialize};
use std::io::Write;

pub(super) struct Serializer<'a, W> {
    writer: &'a mut W,

    field_name: &'static str,
}

impl<'a, W> Serializer<'a, W> {
    pub(super) fn new(writer: &'a mut W, field_name: &'static str) -> Self {
        Self { writer, field_name }
    }
}

impl<'a, W> Serializer<'a, W>
where
    W: Write,
{
    fn write_tag_unescaped(&mut self, tag_name: &[u8], value: &[u8]) {
        self.writer.write_all(b"#");
        self.writer
            .write_all(&Escaper::new(tag_name).collect::<Vec<_>>());
        self.writer.write_all(b":");
        self.writer.write_all(value);
        self.writer.write_all(b";\n");
    }

    fn write_tag(&mut self, tag_name: &[u8], value: &[u8]) {
        self.writer.write_all(b"#");
        self.writer
            .write_all(&Escaper::new(tag_name).collect::<Vec<_>>());
        self.writer.write_all(b":");
        self.writer
            .write_all(&Escaper::new(value).collect::<Vec<_>>());
        self.writer.write_all(b";\n");
    }
}

impl<'a, W> ser::Serializer for &'a mut Serializer<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = tuple::Serializer<'a, W>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(self.field_name.as_bytes())?;
        if v {
            self.writer.write_parameter_unescaped(b"true")?;
        } else {
            self.writer.write_parameter_unescaped(b"false")?;
        }
        self.writer.close_tag()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes());

        self.writer.close_tag()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes());

        self.writer.close_tag()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes());

        self.writer.close_tag()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes());

        self.writer.close_tag()
    }

    #[cfg(has_i128)]
    fn serialize_i128(self, v: i128) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes());

        self.writer.close_tag()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes());

        self.writer.close_tag()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes());

        self.writer.close_tag()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes());

        self.writer.close_tag()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes());

        self.writer.close_tag()
    }

    #[cfg(has_u128)]
    fn serialize_u128(self, v: u128) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes());

        self.writer.close_tag()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = ryu::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes());

        self.writer.close_tag()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = ryu::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes());

        self.writer.close_tag()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = [0; 4];
        v.encode_utf8(&mut buffer);
        self.writer.write_parameter_escaped(&buffer[..v.len_utf8()])?;
        
        self.writer.close_tag()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(self.field_name.as_bytes())?;
        self.writer.write_parameter_escaped(v.as_bytes())?;
        self.writer.close_tag()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(self.field_name.as_bytes())?;
        self.writer.write_parameter_escaped(v)?;
        self.writer.close_tag()
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_some<T>(self, v: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        v.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(self.field_name.as_bytes())?;
        self.writer.write_parameter_unescaped(b"")?;
        self.writer.close_tag()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(self.field_name.as_bytes())?;
        self.writer.write_parameter_unescaped(b"")?;
        self.writer.close_tag()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(self.field_name.as_bytes())?;
        self.writer.write_parameter_escaped(variant.as_bytes())?;
        self.writer.close_tag()
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

    fn serialize_tuple(self, len_: usize) -> Result<Self::SerializeTuple> {
        self.writer.write_tag_name_escaped(self.field_name.as_bytes())?;
        Ok(tuple::Serializer::new(self.writer))
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
        Err(Error::UnsupportedType)
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
    use serde_bytes::Bytes;
    use serde_derive::Serialize;

    #[test]
    fn r#true() {
        let mut output = Vec::new();

        assert_ok!(true.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:true;\n");
    }

    #[test]
    fn r#false() {
        let mut output = Vec::new();

        assert_ok!(false.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:false;\n");
    }

    #[test]
    fn i8() {
        let mut output = Vec::new();

        assert_ok!(42i8.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:42;\n");
    }

    #[test]
    fn i16() {
        let mut output = Vec::new();

        assert_ok!(42i16.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:42;\n");
    }

    #[test]
    fn i32() {
        let mut output = Vec::new();

        assert_ok!(42i32.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:42;\n");
    }

    #[test]
    fn i64() {
        let mut output = Vec::new();

        assert_ok!(42i64.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:42;\n");
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128() {
        let mut output = Vec::new();

        assert_ok!(42i128.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:42;\n");
    }

    #[test]
    fn i8_neg() {
        let mut output = Vec::new();

        assert_ok!((-42i8).serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:-42;\n");
    }

    #[test]
    fn i16_neg() {
        let mut output = Vec::new();

        assert_ok!((-42i16).serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:-42;\n");
    }

    #[test]
    fn i32_neg() {
        let mut output = Vec::new();

        assert_ok!((-42i32).serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:-42;\n");
    }

    #[test]
    fn i64_neg() {
        let mut output = Vec::new();

        assert_ok!((-42i64).serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:-42;\n");
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128_neg() {
        let mut output = Vec::new();

        assert_ok!((-42i128).serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:-42;\n");
    }

    #[test]
    fn u8() {
        let mut output = Vec::new();

        assert_ok!(42u8.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:42;\n");
    }

    #[test]
    fn u16() {
        let mut output = Vec::new();

        assert_ok!(42u16.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:42;\n");
    }

    #[test]
    fn u32() {
        let mut output = Vec::new();

        assert_ok!(42u32.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:42;\n");
    }

    #[test]
    fn u64() {
        let mut output = Vec::new();

        assert_ok!(42u64.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:42;\n");
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128() {
        let mut output = Vec::new();

        assert_ok!(42u128.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:42;\n");
    }

    #[test]
    fn char() {
        let mut output = Vec::new();

        assert_ok!('a'.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:a;\n");
    }

    #[test]
    fn char_escape_number_sign() {
        let mut output = Vec::new();

        assert_ok!('#'.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:\\#;\n");
    }

    #[test]
    fn char_escape_colon() {
        let mut output = Vec::new();

        assert_ok!(':'.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:\\:;\n");
    }

    #[test]
    fn char_escape_semicolon() {
        let mut output = Vec::new();

        assert_ok!(';'.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:\\;;\n");
    }

    #[test]
    fn char_escape_backslash() {
        let mut output = Vec::new();

        assert_ok!('\\'.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:\\\\;\n");
    }

    #[test]
    fn char_does_not_escape_forward_slash() {
        let mut output = Vec::new();

        assert_ok!('/'.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:/;\n");
    }

    #[test]
    fn str() {
        let mut output = Vec::new();

        assert_ok!("bar".serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:bar;\n");
    }

    #[test]
    fn str_escape_number_sign() {
        let mut output = Vec::new();

        assert_ok!("ba#r".serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:ba\\#r;\n");
    }

    #[test]
    fn str_escape_colon() {
        let mut output = Vec::new();

        assert_ok!("ba:r".serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:ba\\:r;\n");
    }

    #[test]
    fn str_escape_semicolon() {
        let mut output = Vec::new();

        assert_ok!("ba;r".serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:ba\\;r;\n");
    }

    #[test]
    fn str_escape_backslash() {
        let mut output = Vec::new();

        assert_ok!("ba\\r".serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:ba\\\\r;\n");
    }

    #[test]
    fn str_escape_double_forwardslash() {
        let mut output = Vec::new();

        assert_ok!("ba//r".serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:ba\\/\\/r;\n");
    }

    #[test]
    fn str_do_not_escape_single_forwardslash() {
        let mut output = Vec::new();

        assert_ok!("ba/r".serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:ba/r;\n");
    }

    #[test]
    fn bytes() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"bar").serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:bar;\n");
    }

    #[test]
    fn bytes_escape_number_sign() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"ba#r").serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:ba\\#r;\n");
    }

    #[test]
    fn bytes_escape_colon() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"ba:r").serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:ba\\:r;\n");
    }

    #[test]
    fn bytes_escape_semicolon() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"ba;r").serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:ba\\;r;\n");
    }

    #[test]
    fn bytes_escape_backslash() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"ba\\r").serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:ba\\\\r;\n");
    }

    #[test]
    fn bytes_escape_double_forwardslash() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"ba//r").serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:ba\\/\\/r;\n");
    }

    #[test]
    fn bytes_do_not_escape_single_forwardslash() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"ba/r").serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:ba/r;\n");
    }

    #[test]
    fn none() {
        let mut output = Vec::new();

        assert_ok!(Option::<()>::None.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"");
    }

    #[test]
    fn some() {
        let mut output = Vec::new();

        assert_ok!(Some(42).serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:42;\n");
    }

    #[test]
    fn unit() {
        let mut output = Vec::new();

        assert_ok!(().serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:;\n");
    }

    #[test]
    fn unit_struct() {
        #[derive(Serialize)]
        struct Bar;

        let mut output = Vec::new();

        assert_ok!(Bar.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:;\n");
    }

    #[test]
    fn unit_variant() {
        #[derive(Serialize)]
        enum Enum {
            A,
        }

        let mut output = Vec::new();

        assert_ok!(Enum::A.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:A;\n");
    }

    #[test]
    fn empty_tuple() {
        let mut output = Vec::new();

        assert_ok!(<[(); 0]>::serialize(&[], &mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo;\n");
    }

    #[test]
    fn single_element_tuple() {
        let mut output = Vec::new();

        assert_ok!((42).serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:42;\n");
    }

    #[test]
    fn multiple_element_tuple() {
        let mut output = Vec::new();

        assert_ok!((42, "bar", (), 1.0).serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:42:bar::1.0;\n");
    }

    #[test]
    fn escapes_field_name() {
        let mut output = Vec::new();

        assert_ok!(().serialize(&mut Serializer::new(&mut output, "fo#o")));

        assert_eq!(output, b"#fo\\#o:;\n");
    }
}
