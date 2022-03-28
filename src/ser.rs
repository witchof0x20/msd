use crate::error::{Error, Result};
use alloc::string::String;
use core::{fmt::Display, iter};
use either::Either;
use serde::{ser, Serialize};

pub struct SerializeSeq;

impl ser::SerializeSeq for SerializeSeq {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

pub struct SerializeTuple;

impl ser::SerializeTuple for SerializeTuple {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

pub struct SerializeTupleStruct;

impl ser::SerializeTupleStruct for SerializeTupleStruct {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

pub struct SerializeTupleVariant;

impl ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

pub struct SerializeMap;

impl ser::SerializeMap for SerializeMap {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

pub struct SerializeStruct;

impl ser::SerializeStruct for SerializeStruct {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

pub struct SerializeStructVariant;

impl ser::SerializeStructVariant for SerializeStructVariant {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

pub struct Serializer {
    output: String,
    nesting_level: usize,
}

impl Serializer {
    fn write_tag(&mut self, tag: &str) {
        if self.nesting_level == 0 {
            self.output.push('#');
        }
        self.output.extend(
            tag.chars()
                .flat_map(|c| c.to_uppercase())
                .flat_map(|c| match c {
                    '#' | ':' | ';' | '\\' => Either::Left(iter::once('\\').chain(iter::once(c))),
                    _ => Either::Right(iter::once(c)),
                }),
        );
        self.output.push_str(";\n");
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SerializeSeq;
    type SerializeTuple = SerializeTuple;
    type SerializeTupleStruct = SerializeTupleStruct;
    type SerializeTupleVariant = SerializeTupleVariant;
    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeStruct;
    type SerializeStructVariant = SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        if v {
            self.write_tag("true");
        } else {
            self.write_tag("false");
        }
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.write_tag(s);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.write_tag(s);
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.write_tag(s);
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.write_tag(s);
        Ok(())
    }

    #[cfg(has_i128)]
    fn serialize_i128(self, v: i128) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.write_tag(s);
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.write_tag(s);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.write_tag(s);
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.write_tag(s);
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.write_tag(s);
        Ok(())
    }

    #[cfg(has_u128)]
    fn serialize_u128(self, v: u128) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(v);
        self.write_tag(s);
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        let mut buffer = ryu::Buffer::new();
        let s = buffer.format(v);
        self.write_tag(s);
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        let mut buffer = ryu::Buffer::new();
        let s = buffer.format(v);
        self.write_tag(s);
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        let mut buffer = [0; 4];
        self.write_tag(v.encode_utf8(&mut buffer));
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_some<T>(self, v: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        todo!()
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
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        todo!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        todo!()
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Display,
    {
        todo!()
    }
}

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: String::new(),
        nesting_level: 0,
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

#[cfg(test)]
mod tests {
    use super::to_string;
    use claim::assert_ok_eq;

    #[test]
    fn bool_true() {
        let expected = "#TRUE;\n";
        assert_ok_eq!(to_string(&true), expected);
    }

    #[test]
    fn bool_false() {
        let expected = "#FALSE;\n";
        assert_ok_eq!(to_string(&false), expected);
    }

    #[test]
    fn i8() {
        let expected = "#42;\n";
        assert_ok_eq!(to_string(&42i8), expected);
    }

    #[test]
    fn i16() {
        let expected = "#42;\n";
        assert_ok_eq!(to_string(&42i16), expected);
    }

    #[test]
    fn i32() {
        let expected = "#42;\n";
        assert_ok_eq!(to_string(&42i32), expected);
    }

    #[test]
    fn i64() {
        let expected = "#42;\n";
        assert_ok_eq!(to_string(&42i64), expected);
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128() {
        let expected = "#42;\n";
        assert_ok_eq!(to_string(&42i128), expected);
    }

    #[test]
    fn i8_neg() {
        let expected = "#-42;\n";
        assert_ok_eq!(to_string(&-42i8), expected);
    }

    #[test]
    fn i16_neg() {
        let expected = "#-42;\n";
        assert_ok_eq!(to_string(&-42i16), expected);
    }

    #[test]
    fn i32_neg() {
        let expected = "#-42;\n";
        assert_ok_eq!(to_string(&-42i32), expected);
    }

    #[test]
    fn i64_neg() {
        let expected = "#-42;\n";
        assert_ok_eq!(to_string(&-42i64), expected);
    }

    #[test]
    #[cfg_attr(not(has_i128), ignore)]
    fn i128_neg() {
        let expected = "#-42;\n";
        assert_ok_eq!(to_string(&-42i128), expected);
    }

    #[test]
    fn u8() {
        let expected = "#42;\n";
        assert_ok_eq!(to_string(&42u8), expected);
    }

    #[test]
    fn u16() {
        let expected = "#42;\n";
        assert_ok_eq!(to_string(&42u16), expected);
    }

    #[test]
    fn u32() {
        let expected = "#42;\n";
        assert_ok_eq!(to_string(&42u32), expected);
    }

    #[test]
    fn u64() {
        let expected = "#42;\n";
        assert_ok_eq!(to_string(&42u64), expected);
    }

    #[test]
    #[cfg_attr(not(has_u128), ignore)]
    fn u128() {
        let expected = "#42;\n";
        assert_ok_eq!(to_string(&42u128), expected);
    }

    #[test]
    fn f32() {
        let expected = "#42.0;\n";
        assert_ok_eq!(to_string(&42f32), expected);
    }

    #[test]
    fn f64() {
        let expected = "#42.0;\n";
        assert_ok_eq!(to_string(&42f64), expected);
    }

    #[test]
    fn char() {
        let expected = "#A;\n";
        assert_ok_eq!(to_string(&'A'), expected);
    }

    #[test]
    fn char_uppercase() {
        let expected = "#A;\n";
        assert_ok_eq!(to_string(&'a'), expected);
    }

    #[test]
    fn char_uppercase_complex() {
        let expected = "#SS;\n";
        assert_ok_eq!(to_string(&'ß'), expected);
    }

    #[test]
    fn char_escape_number_sign() {
        let expected = "#\\#;\n";
        assert_ok_eq!(to_string(&'#'), expected);
    }

    #[test]
    fn char_escape_colon() {
        let expected = "#\\:;\n";
        assert_ok_eq!(to_string(&':'), expected);
    }

    #[test]
    fn char_escape_semicolon() {
        let expected = "#\\;;\n";
        assert_ok_eq!(to_string(&';'), expected);
    }

    #[test]
    fn char_escape_backslash() {
        let expected = "#\\\\;\n";
        assert_ok_eq!(to_string(&'\\'), expected);
    }
}
