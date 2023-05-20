use crate::de::{parse::Values, tuple, Error, Result};
use serde::de::{DeserializeSeed, VariantAccess, Visitor};

pub(in crate::de) struct Access<'a, 'b> {
    values: &'a mut Values<'b>,
}

impl<'a, 'b> Access<'a, 'b> {
    pub(in super::super) fn new(values: &'a mut Values<'b>) -> Self {
        Self { values }
    }
}

impl<'a, 'b, 'de> VariantAccess<'de> for Access<'a, 'b> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        // Deserializing a unit variant will always succeed at this point, because there is nothing
        // more to parse.
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(tuple::element::Deserializer::new(self.values))
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(tuple::Access::new(self.values, len))
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Access;
    use crate::de::{parse::Values, Position};
    use claims::{assert_ok, assert_ok_eq};
    use serde::de::{Error, SeqAccess, VariantAccess, Visitor};
    use std::fmt;

    #[test]
    fn unit_variant() {
        let mut values = Values::new(b"", Position::new(0, 0));
        let access = Access::new(&mut values);

        assert_ok!(access.unit_variant());
    }

    #[test]
    fn newtype_variant() {
        let mut values = Values::new(b"42", Position::new(0, 0));
        let access = Access::new(&mut values);

        assert_ok_eq!(access.newtype_variant::<u64>(), 42);
    }

    #[test]
    fn tuple_variant() {
        struct TupleVisitor;

        impl<'de> Visitor<'de> for TupleVisitor {
            type Value = (u64, String, (), f64);

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("(u64, String, (), f64)")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                Ok((
                    seq.next_element()?
                        .ok_or(A::Error::invalid_length(0, &self))?,
                    seq.next_element()?
                        .ok_or(A::Error::invalid_length(1, &self))?,
                    seq.next_element()?
                        .ok_or(A::Error::invalid_length(2, &self))?,
                    seq.next_element()?
                        .ok_or(A::Error::invalid_length(3, &self))?,
                ))
            }
        }

        let mut values = Values::new(b"42:foo::1.2", Position::new(0, 0));
        let access = Access::new(&mut values);

        assert_ok_eq!(
            access.tuple_variant(4, TupleVisitor),
            (42, "foo".to_string(), (), 1.2)
        );
    }
}
