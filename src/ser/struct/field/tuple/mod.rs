mod element;

use crate::ser::{Error, Result, WriteExt};
use serde::{
    ser::{SerializeTuple, SerializeTupleStruct},
    Serialize,
};
use std::io::Write;

pub(in super::super) struct Serializer<'a, W> {
    writer: &'a mut W,
}

impl<'a, W> Serializer<'a, W> {
    pub(super) fn new(writer: &'a mut W) -> Self {
        Self { writer }
    }
}

impl<'a, W> SerializeTuple for Serializer<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut element::Serializer::new(self.writer))
    }

    fn end(self) -> Result<Self::Ok> {
        self.writer.close_tag()
    }
}

impl<'a, W> SerializeTupleStruct for Serializer<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut element::Serializer::new(self.writer))
    }

    fn end(self) -> Result<Self::Ok> {
        self.writer.close_tag()
    }
}

#[cfg(test)]
mod tests {
    use super::Serializer;
    use claim::assert_ok;
    use serde::ser::SerializeTuple;

    #[test]
    fn empty() {
        let mut output = Vec::new();

        let serializer = Serializer::new(&mut output);

        assert_ok!(serializer.end());
        assert_eq!(output, b";\n");
    }

    #[test]
    fn single() {
        let mut output = Vec::new();

        let mut serializer = Serializer::new(&mut output);

        assert_ok!(serializer.serialize_element(&42));
        assert_ok!(serializer.end());
        assert_eq!(output, b":42;\n");
    }

    #[test]
    fn multiple() {
        let mut output = Vec::new();

        let mut serializer = Serializer::new(&mut output);

        assert_ok!(serializer.serialize_element(&42));
        assert_ok!(serializer.serialize_element(&"foo"));
        assert_ok!(serializer.serialize_element(&()));
        assert_ok!(serializer.serialize_element(&1.0));
        assert_ok!(serializer.end());
        assert_eq!(output, b":42:foo::1.0;\n");
    }
}
