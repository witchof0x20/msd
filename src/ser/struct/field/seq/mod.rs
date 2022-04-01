mod element;

use crate::ser::{Error, Result, WriteExt};
use serde::{ser::SerializeSeq, Serialize};
use std::io::Write;

pub(in super::super) struct Serializer<'a, W> {
    writer: &'a mut W,

    escaped_field_name: &'static [u8],
}

impl<'a, W> Serializer<'a, W> {
    pub(super) fn new(writer: &'a mut W, escaped_field_name: &'static [u8]) -> Self {
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
            .write_tag_name_unescaped(self.escaped_field_name)?;
        value.serialize(&mut element::Serializer::new(self.writer))?;
        self.writer.close_tag()
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}
