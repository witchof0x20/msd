mod map;
mod seq;
mod tuple;

use crate::{
    escape::Escaper,
    ser::{Error, Result, WriteExt},
};
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

impl<'a, W> ser::Serializer for &'a mut Serializer<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = seq::Serializer<'a, W>;
    type SerializeTuple = tuple::Serializer<'a, W>;
    type SerializeTupleStruct = tuple::Serializer<'a, W>;
    type SerializeTupleVariant = tuple::Serializer<'a, W>;
    type SerializeMap = map::Serializer<'a, W>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;
        if v {
            self.writer.write_parameter_unescaped(b"true")?;
        } else {
            self.writer.write_parameter_unescaped(b"false")?;
        }
        self.writer.close_tag()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes())?;

        self.writer.close_tag()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes())?;

        self.writer.close_tag()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes())?;

        self.writer.close_tag()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes())?;

        self.writer.close_tag()
    }

    #[cfg(has_i128)]
    fn serialize_i128(self, v: i128) -> Result<Self::Ok> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes())?;

        self.writer.close_tag()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes())?;

        self.writer.close_tag()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes())?;

        self.writer.close_tag()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes())?;

        self.writer.close_tag()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes())?;

        self.writer.close_tag()
    }

    #[cfg(has_u128)]
    fn serialize_u128(self, v: u128) -> Result<Self::Ok> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes())?;

        self.writer.close_tag()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = ryu::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes())?;

        self.writer.close_tag()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = ryu::Buffer::new();
        let s = buffer.format(v);
        self.writer.write_parameter_unescaped(s.as_bytes())?;

        self.writer.close_tag()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;

        let mut buffer = [0; 4];
        v.encode_utf8(&mut buffer);
        self.writer
            .write_parameter_escaped(&buffer[..v.len_utf8()])?;

        self.writer.close_tag()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;
        self.writer.write_parameter_escaped(v.as_bytes())?;
        self.writer.close_tag()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;
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
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;
        self.writer.write_parameter_unescaped(b"")?;
        self.writer.close_tag()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;
        self.writer.write_parameter_unescaped(b"")?;
        self.writer.close_tag()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;
        self.writer.write_parameter_escaped(variant.as_bytes())?;
        self.writer.close_tag()
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
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnsupportedType)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(seq::Serializer::new(
            self.writer,
            Escaper::new(self.field_name.as_bytes()).collect::<Vec<_>>(),
        ))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;
        Ok(tuple::Serializer::new(self.writer))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;
        Ok(tuple::Serializer::new(self.writer))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;
        self.writer.write_parameter_escaped(variant.as_bytes())?;
        Ok(tuple::Serializer::new(self.writer))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.writer
            .write_tag_name_escaped(self.field_name.as_bytes())?;
        self.writer.write_parameter_unescaped(b"\n")?;
        Ok(map::Serializer::new(self.writer))
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
        ser::{SerializeMap, SerializeTupleStruct, SerializeTupleVariant},
        Serialize,
    };
    use serde_bytes::Bytes;
    use serde_derive::Serialize;
    use std::collections::HashMap;

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
    fn newtype_struct() {
        #[derive(Serialize)]
        struct NewtypeStruct(u32);

        let mut output = Vec::new();

        assert_ok!(NewtypeStruct(42).serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:42;\n");
    }

    #[test]
    fn seq_empty() {
        let mut output = Vec::new();

        assert_ok!(Vec::<()>::new().serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"");
    }

    #[test]
    fn seq_units() {
        let mut output = Vec::new();

        assert_ok!(vec![(), (), ()].serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:;\n#foo:;\n#foo:;\n");
    }

    #[test]
    fn seq_primitives() {
        let mut output = Vec::new();

        assert_ok!(vec![1, 2, 3].serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:1;\n#foo:2;\n#foo:3;\n");
    }

    #[test]
    fn seq_tuples() {
        let mut output = Vec::new();

        assert_ok!(
            vec![(1, 'a'), (2, 'b'), (3, 'c')].serialize(&mut Serializer::new(&mut output, "foo"))
        );

        assert_eq!(output, b"#foo:1:a;\n#foo:2:b;\n#foo:3:c;\n");
    }

    #[test]
    fn seq_maps() {
        struct Map {
            a: (&'static str, usize),
            b: (&'static str, usize),
            c: (&'static str, usize),
            d: (&'static str, usize),
        }
        impl Serialize for Map {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry(self.a.0, &self.a.1)?;
                map.serialize_entry(self.b.0, &self.b.1)?;
                map.serialize_entry(self.c.0, &self.c.1)?;
                map.serialize_entry(self.d.0, &self.d.1)?;
                map.end()
            }
        }

        let mut output = Vec::new();

        assert_ok!(vec![
            Map {
                a: ("a", 1),
                b: ("b", 2),
                c: ("c", 3),
                d: ("d", 4)
            },
            Map {
                a: ("e", 5),
                b: ("f", 6),
                c: ("g", 7),
                d: ("h", 8)
            },
            Map {
                a: ("i", 9),
                b: ("j", 10),
                c: ("k", 11),
                d: ("l", 12)
            }
        ]
        .serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:\n   a:1;\n   b:2;\n   c:3;\n   d:4;\n#END;\n#foo:\n   e:5;\n   f:6;\n   g:7;\n   h:8;\n#END;\n#foo:\n   i:9;\n   j:10;\n   k:11;\n   l:12;\n#END;\n");
    }

    #[test]
    fn seq_structs() {
        #[derive(Serialize)]
        struct Struct {
            foo: usize,
            bar: &'static str,
            baz: (),
            qux: Option<f32>,
        }

        let mut output = Vec::new();

        assert_ok!(vec![
            Struct {
                foo: 1,
                bar: "abc",
                baz: (),
                qux: None
            },
            Struct {
                foo: 2,
                bar: "def",
                baz: (),
                qux: Some(1.1),
            },
            Struct {
                foo: 3,
                bar: "ghi",
                baz: (),
                qux: None,
            }
        ]
        .serialize(&mut Serializer::new(&mut output, "repeating")));

        assert_eq!(output, b"#repeating:;\n#foo:1;\n#bar:abc;\n#baz:;\n#repeating:;\n#foo:2;\n#bar:def;\n#baz:;\n#qux:1.1;\n#repeating:;\n#foo:3;\n#bar:ghi;\n#baz:;\n");
    }

    #[test]
    fn seq_struct_variants() {
        #[derive(Serialize)]
        enum Struct {
            Variant {
                foo: usize,
                bar: &'static str,
                baz: (),
                qux: Option<f32>,
            },
        }

        let mut output = Vec::new();

        assert_ok!(vec![
            Struct::Variant {
                foo: 1,
                bar: "abc",
                baz: (),
                qux: None
            },
            Struct::Variant {
                foo: 2,
                bar: "def",
                baz: (),
                qux: Some(1.1),
            },
            Struct::Variant {
                foo: 3,
                bar: "ghi",
                baz: (),
                qux: None,
            }
        ]
        .serialize(&mut Serializer::new(&mut output, "repeating")));

        assert_eq!(output, b"#repeating:Variant;\n#foo:1;\n#bar:abc;\n#baz:;\n#repeating:Variant;\n#foo:2;\n#bar:def;\n#baz:;\n#qux:1.1;\n#repeating:Variant;\n#foo:3;\n#bar:ghi;\n#baz:;\n");
    }

    #[test]
    fn empty_tuple() {
        let mut output = Vec::new();

        assert_ok!(<[(); 0]>::serialize(
            &[],
            &mut Serializer::new(&mut output, "foo")
        ));

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
    fn nested_tuple() {
        let mut output = Vec::new();

        assert_ok!((1, (2, 3), ((4), 5), 6).serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:1:2:3:4:5:6;\n");
    }

    #[test]
    fn empty_tuple_struct() {
        #[derive(Serialize)]
        struct TupleStruct();

        let mut output = Vec::new();

        assert_ok!(TupleStruct().serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo;\n");
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

        assert_ok!(TupleStruct(42).serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:42;\n");
    }

    #[test]
    fn multiple_element_tuple_struct() {
        #[derive(Serialize)]
        struct TupleStruct(usize, &'static str, (), f32);

        let mut output = Vec::new();

        assert_ok!(
            TupleStruct(42, "bar", (), 1.0).serialize(&mut Serializer::new(&mut output, "foo"))
        );

        assert_eq!(output, b"#foo:42:bar::1.0;\n");
    }

    #[test]
    fn nested_tuple_struct() {
        #[derive(Serialize)]
        struct TupleStruct(usize, (usize, usize), ((usize, usize), usize), usize);

        let mut output = Vec::new();

        assert_ok!(TupleStruct(1, (2, 3), ((4, 5), 6), 7)
            .serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:1:2:3:4:5:6:7;\n");
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

        assert_ok!(TupleEnum::Variant().serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:Variant;\n");
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
                let mut tv =
                    serializer.serialize_tuple_variant("TupleEnum", 0, "Variant", 1)?;
                tv.serialize_field(&inner)?;
                tv.end()
            }
        }

        let mut output = Vec::new();

        assert_ok!(TupleEnum::Variant(42).serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:Variant:42;\n");
    }

    #[test]
    fn multiple_element_tuple_variant() {
        #[derive(Serialize)]
        enum TupleEnum {
            Variant(usize, &'static str, (), f32),
        }

        let mut output = Vec::new();

        assert_ok!(TupleEnum::Variant(42, "bar", (), 1.0)
            .serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:Variant:42:bar::1.0;\n");
    }

    #[test]
    fn nested_tuple_variant() {
        #[derive(Serialize)]
        enum TupleEnum {
            Variant(usize, (usize, usize), ((usize, usize), usize), usize),
        }

        let mut output = Vec::new();

        assert_ok!(TupleEnum::Variant(1, (2, 3), ((4, 5), 6), 7)
            .serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:Variant:1:2:3:4:5:6:7;\n");
    }

    #[test]
    fn empty_map() {
        let map: HashMap<(), ()> = HashMap::new();

        let mut output = Vec::new();

        assert_ok!(map.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:\n#END;\n");
    }

    #[test]
    fn single_entry_map() {
        let mut map = HashMap::new();
        map.insert("abc", 1);

        let mut output = Vec::new();

        assert_ok!(map.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(output, b"#foo:\n   abc:1;\n#END;\n");
    }

    #[test]
    fn multiple_entry_map() {
        struct Map;
        impl Serialize for Map {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry("abc", &1)?;
                map.serialize_entry("def", &2)?;
                map.serialize_entry("ghi", &3)?;
                map.serialize_entry("jkl", &4)?;
                map.end()
            }
        }

        let mut output = Vec::new();

        assert_ok!(Map.serialize(&mut Serializer::new(&mut output, "foo")));

        assert_eq!(
            output,
            b"#foo:\n   abc:1;\n   def:2;\n   ghi:3;\n   jkl:4;\n#END;\n"
        );
    }

    #[test]
    fn escapes_field_name() {
        let mut output = Vec::new();

        assert_ok!(().serialize(&mut Serializer::new(&mut output, "fo#o")));

        assert_eq!(output, b"#fo\\#o:;\n");
    }
}
