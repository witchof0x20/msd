mod error;
mod escaped;
mod map;
mod seq;
mod r#struct;
mod tuple;
mod write;

pub use error::{Error, Result};

use escaped::Escaped;
use serde::{ser, Serialize};
use std::io::Write;
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
    type SerializeSeq = seq::tag::Serializer<'a, W>;
    type SerializeTuple = tuple::tag::Serializer<'a, W>;
    type SerializeTupleStruct = tuple::tag::Serializer<'a, W>;
    type SerializeTupleVariant = tuple::Serializer<'a, W>;
    type SerializeMap = map::tag::Serializer<'a, W>;
    type SerializeStruct = r#struct::Serializer<'a, W>;
    type SerializeStructVariant = r#struct::Serializer<'a, W>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        if v {
            self.writer.write_tag_name_unescaped(b"true")?;
        } else {
            self.writer.write_tag_name_unescaped(b"false")?;
        }
        self.writer.close_tag()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        self.writer
            .write_tag_name_unescaped(buffer.format(v).as_bytes())?;
        self.writer.close_tag()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        self.writer
            .write_tag_name_unescaped(buffer.format(v).as_bytes())?;
        self.writer.close_tag()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        self.writer
            .write_tag_name_unescaped(buffer.format(v).as_bytes())?;
        self.writer.close_tag()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        self.writer
            .write_tag_name_unescaped(buffer.format(v).as_bytes())?;
        self.writer.close_tag()
    }

    #[cfg(has_i128)]
    fn serialize_i128(self, v: i128) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        self.writer
            .write_tag_name_unescaped(buffer.format(v).as_bytes())?;
        self.writer.close_tag()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        self.writer
            .write_tag_name_unescaped(buffer.format(v).as_bytes())?;
        self.writer.close_tag()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        self.writer
            .write_tag_name_unescaped(buffer.format(v).as_bytes())?;
        self.writer.close_tag()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        self.writer
            .write_tag_name_unescaped(buffer.format(v).as_bytes())?;
        self.writer.close_tag()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        self.writer
            .write_tag_name_unescaped(buffer.format(v).as_bytes())?;
        self.writer.close_tag()
    }

    #[cfg(has_i128)]
    fn serialize_u128(self, v: u128) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        self.writer
            .write_tag_name_unescaped(buffer.format(v).as_bytes())?;
        self.writer.close_tag()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        let mut buffer = ryu::Buffer::new();
        self.writer
            .write_tag_name_unescaped(buffer.format(v).as_bytes())?;
        self.writer.close_tag()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        let mut buffer = ryu::Buffer::new();
        self.writer
            .write_tag_name_unescaped(buffer.format(v).as_bytes())?;
        self.writer.close_tag()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        let mut buffer = [0; 4];
        v.encode_utf8(&mut buffer);
        self.writer
            .write_tag_name_escaped(&buffer[..v.len_utf8()])?;
        self.writer.close_tag()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(v.as_bytes())?;
        self.writer.close_tag()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(v)?;
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
        self.writer.write_tag_name_unescaped(b"")?;
        self.writer.close_tag()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.writer.write_tag_name_unescaped(b"")?;
        self.writer.close_tag()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.writer.write_tag_name_escaped(variant.as_bytes())?;
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
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(r#struct::field::Serializer::new(
            &mut self.writer,
            Escaped::new(variant.as_bytes()).collect::<Vec<_>>(),
        ))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(seq::tag::Serializer::new(&mut self.writer))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(tuple::tag::Serializer::new(&mut self.writer))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(tuple::tag::Serializer::new(&mut self.writer))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.writer.write_tag_name_escaped(variant.as_bytes())?;
        Ok(tuple::Serializer::new(&mut self.writer))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(map::tag::Serializer::new(&mut self.writer))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(r#struct::Serializer::new(&mut self.writer))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.writer.write_tag_name_escaped(variant.as_bytes())?;
        self.writer.close_tag()?;
        Ok(r#struct::Serializer::new(&mut self.writer))
    }
}

pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: Write,
    T: ?Sized + Serialize,
{
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer)?;
    Ok(())
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut bytes = Vec::with_capacity(128);
    to_writer(&mut bytes, value)?;
    Ok(bytes)
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

        assert_ok!(true.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#true;\n");
    }

    #[test]
    fn r#false() {
        let mut output = Vec::new();

        assert_ok!(false.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#false;\n");
    }

    #[test]
    fn i8() {
        let mut output = Vec::new();

        assert_ok!(42i8.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#42;\n");
    }

    #[test]
    fn i16() {
        let mut output = Vec::new();

        assert_ok!(42i16.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#42;\n");
    }

    #[test]
    fn i32() {
        let mut output = Vec::new();

        assert_ok!(42i32.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#42;\n");
    }

    #[test]
    fn i64() {
        let mut output = Vec::new();

        assert_ok!(42i64.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#42;\n");
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128() {
        let mut output = Vec::new();

        assert_ok!(42i128.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#42;\n");
    }

    #[test]
    fn i8_neg() {
        let mut output = Vec::new();

        assert_ok!((-42i8).serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#-42;\n");
    }

    #[test]
    fn i16_neg() {
        let mut output = Vec::new();

        assert_ok!((-42i16).serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#-42;\n");
    }

    #[test]
    fn i32_neg() {
        let mut output = Vec::new();

        assert_ok!((-42i32).serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#-42;\n");
    }

    #[test]
    fn i64_neg() {
        let mut output = Vec::new();

        assert_ok!((-42i64).serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#-42;\n");
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128_neg() {
        let mut output = Vec::new();

        assert_ok!((-42i128).serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#-42;\n");
    }

    #[test]
    fn u8() {
        let mut output = Vec::new();

        assert_ok!(42u8.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#42;\n");
    }

    #[test]
    fn u16() {
        let mut output = Vec::new();

        assert_ok!(42u16.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#42;\n");
    }

    #[test]
    fn u32() {
        let mut output = Vec::new();

        assert_ok!(42u32.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#42;\n");
    }

    #[test]
    fn u64() {
        let mut output = Vec::new();

        assert_ok!(42u64.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#42;\n");
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn u128() {
        let mut output = Vec::new();

        assert_ok!(42u128.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#42;\n");
    }

    #[test]
    fn f32() {
        let mut output = Vec::new();

        assert_ok!(42f32.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#42.0;\n");
    }

    #[test]
    fn f64() {
        let mut output = Vec::new();

        assert_ok!(42f64.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#42.0;\n");
    }

    #[test]
    fn char() {
        let mut output = Vec::new();

        assert_ok!('a'.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#a;\n");
    }

    #[test]
    fn char_escape_number_sign() {
        let mut output = Vec::new();

        assert_ok!('#'.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#\\#;\n");
    }

    #[test]
    fn char_escape_colon() {
        let mut output = Vec::new();

        assert_ok!(':'.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#\\:;\n");
    }

    #[test]
    fn char_escape_semicolon() {
        let mut output = Vec::new();

        assert_ok!(';'.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#\\;;\n");
    }

    #[test]
    fn char_escape_backslash() {
        let mut output = Vec::new();

        assert_ok!('\\'.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#\\\\;\n");
    }

    #[test]
    fn char_does_not_escape_forward_slash() {
        let mut output = Vec::new();

        assert_ok!('/'.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#/;\n");
    }

    #[test]
    fn str() {
        let mut output = Vec::new();

        assert_ok!("bar".serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#bar;\n");
    }

    #[test]
    fn str_escape_number_sign() {
        let mut output = Vec::new();

        assert_ok!("ba#r".serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#ba\\#r;\n");
    }

    #[test]
    fn str_escape_colon() {
        let mut output = Vec::new();

        assert_ok!("ba:r".serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#ba\\:r;\n");
    }

    #[test]
    fn str_escape_semicolon() {
        let mut output = Vec::new();

        assert_ok!("ba;r".serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#ba\\;r;\n");
    }

    #[test]
    fn str_escape_backslash() {
        let mut output = Vec::new();

        assert_ok!("ba\\r".serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#ba\\\\r;\n");
    }

    #[test]
    fn str_escape_double_forwardslash() {
        let mut output = Vec::new();

        assert_ok!("ba//r".serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#ba\\/\\/r;\n");
    }

    #[test]
    fn str_do_not_escape_single_forwardslash() {
        let mut output = Vec::new();

        assert_ok!("ba/r".serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#ba/r;\n");
    }

    #[test]
    fn bytes() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"bar").serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#bar;\n");
    }

    #[test]
    fn bytes_escape_number_sign() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"ba#r").serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#ba\\#r;\n");
    }

    #[test]
    fn bytes_escape_colon() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"ba:r").serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#ba\\:r;\n");
    }

    #[test]
    fn bytes_escape_semicolon() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"ba;r").serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#ba\\;r;\n");
    }

    #[test]
    fn bytes_escape_backslash() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"ba\\r").serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#ba\\\\r;\n");
    }

    #[test]
    fn bytes_escape_double_forwardslash() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"ba//r").serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#ba\\/\\/r;\n");
    }

    #[test]
    fn bytes_do_not_escape_single_forwardslash() {
        let mut output = Vec::new();

        assert_ok!(Bytes::new(b"ba/r").serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#ba/r;\n");
    }

    #[test]
    fn none() {
        let mut output = Vec::new();

        assert_ok!(Option::<()>::None.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"");
    }

    #[test]
    fn some() {
        let mut output = Vec::new();

        assert_ok!(Some(42).serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#42;\n");
    }

    #[test]
    fn unit() {
        let mut output = Vec::new();

        assert_ok!(().serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#;\n");
    }

    #[test]
    fn unit_struct() {
        #[derive(Serialize)]
        struct Unit;

        let mut output = Vec::new();

        assert_ok!(Unit.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#;\n");
    }

    #[test]
    fn unit_variant() {
        #[derive(Serialize)]
        enum Unit {
            Variant,
        }

        let mut output = Vec::new();

        assert_ok!(Unit::Variant.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#Variant;\n");
    }

    #[test]
    fn newtype_struct() {
        #[derive(Serialize)]
        struct Foo {
            bar: usize,
            baz: Option<()>,
            qux: Option<&'static str>,
        }
        #[derive(Serialize)]
        struct Newtype(Foo);
        let mut output = Vec::new();

        assert_ok!(Newtype(Foo {
            bar: 42,
            baz: None,
            qux: Some("test:test"),
        })
        .serialize(&mut Serializer::new(&mut output)));
        assert_eq!(output, b"#bar:42;\n#qux:test\\:test;\n");
    }

    #[test]
    fn newtype_variant() {
        #[derive(Serialize)]
        enum Newtype {
            Variant(usize),
        }
        let mut output = Vec::new();

        assert_ok!(Newtype::Variant(42).serialize(&mut Serializer::new(&mut output)));
        assert_eq!(output, b"#Variant:42;\n");
    }

    #[test]
    fn seq_empty() {
        let mut output = Vec::new();

        assert_ok!(Vec::<()>::new().serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"");
    }

    #[test]
    fn seq_units() {
        let mut output = Vec::new();

        assert_ok!(vec![(), (), ()].serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#;\n#;\n#;\n");
    }

    #[test]
    fn seq_primitives() {
        let mut output = Vec::new();

        assert_ok!(vec![1, 2, 3].serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#1;\n#2;\n#3;\n");
    }

    #[test]
    fn seq_tuples() {
        let mut output = Vec::new();

        assert_ok!(vec![(1, 'a'), (2, 'b'), (3, 'c')].serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#1:a;\n#2:b;\n#3:c;\n");
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
        .serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#Variant:;\n#foo:1;\n#bar:abc;\n#baz:;\n#Variant:;\n#foo:2;\n#bar:def;\n#baz:;\n#qux:1.1;\n#Variant:;\n#foo:3;\n#bar:ghi;\n#baz:;\n");
    }

    #[test]
    fn empty_tuple() {
        let mut output = Vec::new();

        assert_ok!(<[(); 0]>::serialize(&[], &mut Serializer::new(&mut output)));

        assert_eq!(output, b"#;\n");
    }

    #[test]
    fn single_element_tuple() {
        let mut output = Vec::new();

        assert_ok!((42).serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#42;\n");
    }

    #[test]
    fn multiple_element_tuple() {
        let mut output = Vec::new();

        assert_ok!((42, "bar", (), 1.0).serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#42:bar::1.0;\n");
    }

    #[test]
    fn nested_tuple() {
        let mut output = Vec::new();

        assert_ok!((1, (2, 3), ((4), 5), 6).serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#1:2:3:4:5:6;\n");
    }

    #[test]
    fn empty_tuple_struct() {
        #[derive(Serialize)]
        struct TupleStruct();

        let mut output = Vec::new();

        assert_ok!(TupleStruct().serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#;\n");
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

        assert_ok!(TupleStruct(42).serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#42;\n");
    }

    #[test]
    fn multiple_element_tuple_struct() {
        #[derive(Serialize)]
        struct TupleStruct(usize, &'static str, (), f32);

        let mut output = Vec::new();

        assert_ok!(TupleStruct(42, "bar", (), 1.0).serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#42:bar::1.0;\n");
    }

    #[test]
    fn nested_tuple_struct() {
        #[derive(Serialize)]
        struct TupleStruct(usize, (usize, usize), ((usize, usize), usize), usize);

        let mut output = Vec::new();

        assert_ok!(
            TupleStruct(1, (2, 3), ((4, 5), 6), 7).serialize(&mut Serializer::new(&mut output))
        );

        assert_eq!(output, b"#1:2:3:4:5:6:7;\n");
    }

    #[test]
    fn empty_tuple_variant() {
        #[derive(Serialize)]
        enum Tuple {
            Variant(),
        }

        let mut output = Vec::new();

        assert_ok!(Tuple::Variant().serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#Variant;\n");
    }

    #[test]
    fn single_element_tuple_variant() {
        enum Tuple {
            Variant(usize),
        }
        impl Serialize for Tuple {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let Self::Variant(inner) = self;
                let mut tv = serializer.serialize_tuple_variant("TupleStruct", 0, "Variant", 1)?;
                tv.serialize_field(&inner)?;
                tv.end()
            }
        }

        let mut output = Vec::new();

        assert_ok!(Tuple::Variant(42).serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#Variant:42;\n");
    }

    #[test]
    fn multiple_element_tuple_variant() {
        #[derive(Serialize)]
        enum Tuple {
            Variant(usize, &'static str, (), f32),
        }

        let mut output = Vec::new();

        assert_ok!(Tuple::Variant(42, "bar", (), 1.0).serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#Variant:42:bar::1.0;\n");
    }

    #[test]
    fn nested_tuple_variant() {
        #[derive(Serialize)]
        enum Tuple {
            Variant(usize, (usize, usize), ((usize, usize), usize), usize),
        }

        let mut output = Vec::new();

        assert_ok!(
            Tuple::Variant(1, (2, 3), ((4, 5), 6), 7).serialize(&mut Serializer::new(&mut output))
        );

        assert_eq!(output, b"#Variant:1:2:3:4:5:6:7;\n");
    }

    #[test]
    fn empty_map() {
        let map: HashMap<(), ()> = HashMap::new();

        let mut output = Vec::new();

        assert_ok!(map.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#;\n");
    }

    #[test]
    fn single_entry_map() {
        let mut map = HashMap::new();
        map.insert("abc", 1);

        let mut output = Vec::new();

        assert_ok!(map.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#abc:1;\n");
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

        assert_ok!(Map.serialize(&mut Serializer::new(&mut output)));

        assert_eq!(output, b"#abc:1;\n#def:2;\n#ghi:3;\n#jkl:4;\n");
    }

    #[test]
    fn struct_empty() {
        #[derive(Serialize)]
        struct Foo {}
        let mut output = Vec::new();

        assert_ok!(Foo {}.serialize(&mut Serializer::new(&mut output)));
        assert_eq!(output, b"");
    }

    #[test]
    fn struct_single_field() {
        #[derive(Serialize)]
        struct Foo {
            bar: usize,
        }
        let mut output = Vec::new();

        assert_ok!(Foo { bar: 42 }.serialize(&mut Serializer::new(&mut output)));
        assert_eq!(output, b"#bar:42;\n");
    }

    #[test]
    fn struct_multiple_fields() {
        #[derive(Serialize)]
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

    #[test]
    fn struct_variant_empty() {
        #[derive(Serialize)]
        enum Foo {
            Variant {},
        }
        let mut output = Vec::new();

        assert_ok!(Foo::Variant {}.serialize(&mut Serializer::new(&mut output)));
        assert_eq!(output, b"#Variant;\n");
    }

    #[test]
    fn struct_variant_single_field() {
        #[derive(Serialize)]
        enum Foo {
            Variant { bar: usize },
        }
        let mut output = Vec::new();

        assert_ok!(Foo::Variant { bar: 42 }.serialize(&mut Serializer::new(&mut output)));
        assert_eq!(output, b"#Variant;\n#bar:42;\n");
    }

    #[test]
    fn struct_variant_multiple_fields() {
        #[derive(Serialize)]
        enum Foo {
            Variant {
                bar: usize,
                baz: Option<()>,
                qux: Option<&'static str>,
            },
        }
        let mut output = Vec::new();

        assert_ok!(Foo::Variant {
            bar: 42,
            baz: None,
            qux: Some("test:test"),
        }
        .serialize(&mut Serializer::new(&mut output)));
        assert_eq!(output, b"#Variant;\n#bar:42;\n#qux:test\\:test;\n");
    }
}
