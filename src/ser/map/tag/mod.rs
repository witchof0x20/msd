mod key;

use super::value;
use crate::ser::{Error, Result, WriteExt};
use serde::{ser::SerializeMap, Serialize};
use std::io::Write;

pub struct Serializer<'a, W> {
    writer: &'a mut W,
    written_field: bool,
}

impl<'a, W> Serializer<'a, W> {
    pub(in super::super) fn new(writer: &'a mut W) -> Self {
        Self { writer, written_field: false }
    }
}

impl<'a, W> SerializeMap for Serializer<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.written_field = true;
        key.serialize(key::Serializer::new(self.writer))
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(value::Serializer::new(self.writer))
    }

    fn end(self) -> Result<Self::Ok> {
        if !self.written_field {
            self.writer.write_tag_name_unescaped(b"")?;
            self.writer.close_tag()
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Serializer;
    use claim::assert_ok;
    use serde::ser::SerializeMap;

    #[test]
    fn empty() {
        let mut output = Vec::new();
        let serializer = Serializer::new(&mut output);

        assert_ok!(serializer.end());
        assert_eq!(output, b"#;\n");
    }

    #[test]
    fn single_entry() {
        let mut output = Vec::new();
        let mut serializer = Serializer::new(&mut output);

        assert_ok!(serializer.serialize_key("foo"));
        assert_ok!(serializer.serialize_value(&42));
        assert_ok!(serializer.end());

        assert_eq!(output, b"#foo:42;\n");
    }

    #[test]
    fn multiple_elements() {
        let mut output = Vec::new();
        let mut serializer = Serializer::new(&mut output);

        assert_ok!(serializer.serialize_key("foo"));
        assert_ok!(serializer.serialize_value(&1));
        assert_ok!(serializer.serialize_key("bar"));
        assert_ok!(serializer.serialize_value(&2));
        assert_ok!(serializer.end());

        assert_eq!(output, b"#foo:1;\n#bar:2;\n");
    }
}
