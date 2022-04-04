mod element;

use crate::ser::{Error, Result};
use serde::{ser::SerializeSeq, Serialize};
use std::io::Write;

pub struct Serializer<'a, W> {
    writer: &'a mut W,
}

impl<'a, W> Serializer<'a, W> {
    pub(in super::super) fn new(writer: &'a mut W) -> Self {
        Self {
            writer,
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
        value.serialize(&mut element::Serializer::new(self.writer))
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
        let serializer = Serializer::new(&mut output);

        assert_ok!(serializer.end());
        assert_eq!(output, b"");
    }

    #[test]
    fn single_element() {
        let mut output = Vec::new();
        let mut serializer = Serializer::new(&mut output);

        assert_ok!(serializer.serialize_element(&42));
        assert_ok!(serializer.end());

        assert_eq!(output, b"#42;\n");
    }

    #[test]
    fn multiple_elements() {
        let mut output = Vec::new();
        let mut serializer = Serializer::new(&mut output);

        assert_ok!(serializer.serialize_element(&42));
        assert_ok!(serializer.serialize_element(&"bar"));
        assert_ok!(serializer.serialize_element(&()));
        assert_ok!(serializer.serialize_element::<Option<()>>(&None));
        assert_ok!(serializer.end());

        assert_eq!(output, b"#42;\n#bar;\n#;\n#;\n");
    }
}
