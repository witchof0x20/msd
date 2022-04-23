mod variant;

use crate::de::{parse::Values, Error, Result};
use serde::de::{DeserializeSeed, EnumAccess};

pub(in crate::de) struct Access<'a, 'b> {
    values: &'a mut Values<'b>,
}

impl<'a, 'b> Access<'a, 'b> {
    pub(in crate::de) fn new(values: &'a mut Values<'b>) -> Self {
        Self { values }
    }
}

impl<'a, 'b, 'de> EnumAccess<'de> for Access<'a, 'b> {
    type Error = Error;
    type Variant = variant::Access<'a, 'b>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        Ok((
            seed.deserialize(variant::Deserializer::new(self.values.next()?))?,
            variant::Access::new(self.values),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::Access;
    use crate::de::parse::Values;
    use claim::assert_ok;
    use serde::{
        de,
        de::{EnumAccess, Visitor},
        Deserialize,
    };
    use std::fmt;

    #[test]
    fn variant() {
        #[derive(Debug, PartialEq)]
        struct Variant(String);
        impl<'de> Deserialize<'de> for Variant {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                struct VariantVisitor;

                impl<'de> Visitor<'de> for VariantVisitor {
                    type Value = Variant;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("identifier")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Ok(Variant(value.to_owned()))
                    }
                }

                deserializer.deserialize_identifier(VariantVisitor)
            }
        }

        let mut values = Values::new(b"foo", 0, 0);
        let access = Access::new(&mut values);

        let (variant, _variant_access) = assert_ok!(access.variant::<Variant>());
        assert_eq!(variant, Variant("foo".to_string()));
    }
}
