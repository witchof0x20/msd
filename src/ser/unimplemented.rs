use crate::{Error, Result};
use serde::{ser, Serialize};

pub struct Unimplemented;

impl ser::SerializeSeq for Unimplemented {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!("only struct serialization is currently implemented")
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!("only struct serialization is currently implemented")
    }
}

impl ser::SerializeTuple for Unimplemented {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!("only struct serialization is currently implemented")
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!("only struct serialization is currently implemented")
    }
}

impl ser::SerializeTupleStruct for Unimplemented {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!("only struct serialization is currently implemented")
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!("only struct serialization is currently implemented")
    }
}

impl ser::SerializeTupleVariant for Unimplemented {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!("only struct serialization is currently implemented")
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!("only struct serialization is currently implemented")
    }
}

impl ser::SerializeMap for Unimplemented {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!("only struct serialization is currently implemented")
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!("only struct serialization is currently implemented")
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!("only struct serialization is currently implemented")
    }
}

impl ser::SerializeStruct for Unimplemented {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!("only struct serialization is currently implemented")
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!("only struct serialization is currently implemented")
    }
}

impl ser::SerializeStructVariant for Unimplemented {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!("only struct serialization is currently implemented")
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!("only struct serialization is currently implemented")
    }
}
