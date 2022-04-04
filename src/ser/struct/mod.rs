pub(super) mod field;

use crate::ser::{Error, Escaped, Result};
use serde::{
    ser::{SerializeStruct, SerializeStructVariant},
    Serialize,
};
use std::io::Write;

pub struct Serializer<'a, W> {
    writer: &'a mut W,
}

impl<'a, W> Serializer<'a, W> {
    pub(super) fn new(writer: &'a mut W) -> Self {
        Self { writer }
    }
}

impl<'a, W> SerializeStruct for Serializer<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut field::Serializer::new(
            self.writer,
            Escaped::new(key.as_bytes()).collect::<Vec<_>>(),
        ))
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W> SerializeStructVariant for Serializer<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut field::Serializer::new(
            self.writer,
            Escaped::new(key.as_bytes()).collect::<Vec<_>>(),
        ))
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Serializer;
    use claim::assert_ok;

    #[test]
    fn struct_no_fields() {
        use serde::ser::SerializeStruct;

        let mut output = Vec::new();
        let serializer = Serializer::new(&mut output);

        assert_ok!(serializer.end());
        assert_eq!(output, b"");
    }

    #[test]
    fn struct_field() {
        use serde::ser::SerializeStruct;

        let mut output = Vec::new();
        let mut serializer = Serializer::new(&mut output);

        assert_ok!(serializer.serialize_field("foo", &42));

        assert_ok!(serializer.end());
        assert_eq!(output, b"#foo:42;\n");
    }

    #[test]
    fn struct_multiple_fields() {
        use serde::ser::SerializeStruct;

        let mut output = Vec::new();
        let mut serializer = Serializer::new(&mut output);

        assert_ok!(serializer.serialize_field("foo", &42));
        assert_ok!(serializer.serialize_field("bar", &Option::<()>::None));
        assert_ok!(serializer.serialize_field("baz", &"test;"));

        assert_ok!(serializer.end());
        assert_eq!(output, b"#foo:42;\n#baz:test\\;;\n");
    }

    #[test]
    fn struct_variant_no_fields() {
        use serde::ser::SerializeStructVariant;

        let mut output = Vec::new();
        let serializer = Serializer::new(&mut output);

        assert_ok!(serializer.end());
        assert_eq!(output, b"");
    }

    #[test]
    fn struct_variant_field() {
        use serde::ser::SerializeStructVariant;

        let mut output = Vec::new();
        let mut serializer = Serializer::new(&mut output);

        assert_ok!(serializer.serialize_field("foo", &42));

        assert_ok!(serializer.end());
        assert_eq!(output, b"#foo:42;\n");
    }

    #[test]
    fn struct_variant_multiple_fields() {
        use serde::ser::SerializeStructVariant;

        let mut output = Vec::new();
        let mut serializer = Serializer::new(&mut output);

        assert_ok!(serializer.serialize_field("foo", &42));
        assert_ok!(serializer.serialize_field("bar", &Option::<()>::None));
        assert_ok!(serializer.serialize_field("baz", &"test;"));

        assert_ok!(serializer.end());
        assert_eq!(output, b"#foo:42;\n#baz:test\\;;\n");
    }
}
