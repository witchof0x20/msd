pub(super) mod tag;

mod element;

use crate::ser::{Error, Result, WriteExt};
use serde::{ser::SerializeSeq, Serialize};
use std::io::Write;

pub(in super::super) struct Serializer<'a, W> {
    writer: &'a mut W,

    escaped_field_name: Vec<u8>,
}

impl<'a, W> Serializer<'a, W> {
    pub(super) fn new(writer: &'a mut W, escaped_field_name: Vec<u8>) -> Self {
        Self {
            writer,

            escaped_field_name,
        }
    }
}

impl<'a, W> SerializeSeq for Serializer<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.writer
            .write_tag_name_unescaped(&self.escaped_field_name)?;
        value.serialize(element::Serializer::new(self.writer))
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Serializer;
    use claim::assert_ok;
    use serde::ser::SerializeSeq;

    #[test]
    fn empty() {
        let mut output = Vec::new();
        let serializer = Serializer::new(&mut output, b"foo".to_vec());

        assert_ok!(serializer.end());
        assert_eq!(output, b"");
    }

    #[test]
    fn single_element() {
        let mut output = Vec::new();
        let mut serializer = Serializer::new(&mut output, b"foo".to_vec());

        assert_ok!(serializer.serialize_element(&42));
        assert_ok!(serializer.end());

        assert_eq!(output, b"#foo:42;\n");
    }

    #[test]
    fn multiple_elements() {
        let mut output = Vec::new();
        let mut serializer = Serializer::new(&mut output, b"foo".to_vec());

        assert_ok!(serializer.serialize_element(&42));
        assert_ok!(serializer.serialize_element(&"bar"));
        assert_ok!(serializer.serialize_element(&()));
        assert_ok!(serializer.end());

        assert_eq!(output, b"#foo:42;\n#foo:bar;\n#foo:;\n");
    }
}
