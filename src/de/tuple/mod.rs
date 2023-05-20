pub(in crate::de) mod element;

use super::{parse::Values, Error, Result};
use serde::de::{DeserializeSeed, SeqAccess};

pub(in crate::de) struct Access<'a, 'b> {
    values: &'a mut Values<'b>,
    len: usize,
}

impl<'a, 'b> Access<'a, 'b> {
    pub(in crate::de) fn new(values: &'a mut Values<'b>, len: usize) -> Self {
        Self { values, len }
    }
}

impl<'a, 'b, 'de> SeqAccess<'de> for Access<'a, 'b> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        self.len = self.len.saturating_sub(1);
        Ok(Some(
            seed.deserialize(element::Deserializer::new(self.values))?,
        ))
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

#[cfg(test)]
mod tests {
    use super::Access;
    use crate::de::{error, parse::Values, Error, Position};
    use claims::{assert_err_eq, assert_ok, assert_some_eq};
    use serde::de::SeqAccess;

    #[test]
    fn empty() {
        let mut values = Values::new(b"", Position::new(0, 0));
        // Consume the single value, as all values are non-empty.
        assert_ok!(values.next());
        assert_ok!(values.assert_exhausted());
        let mut access = Access::new(&mut values, 0);

        assert_some_eq!(access.size_hint(), 0);
        assert_err_eq!(
            access.next_element::<bool>(),
            Error::new(error::Kind::EndOfValues, Position::new(0, 0))
        );
        assert_some_eq!(access.size_hint(), 0);
    }

    #[test]
    fn one_value() {
        let mut values = Values::new(b"42", Position::new(0, 0));
        let mut access = Access::new(&mut values, 1);

        assert_some_eq!(access.size_hint(), 1);
        assert_some_eq!(assert_ok!(access.next_element::<u64>()), 42);
        assert_some_eq!(access.size_hint(), 0);
        assert_err_eq!(
            access.next_element::<bool>(),
            Error::new(error::Kind::EndOfValues, Position::new(0, 2))
        );
        assert_some_eq!(access.size_hint(), 0);
    }

    #[test]
    fn multiple_values() {
        let mut values = Values::new(b"foo:42", Position::new(0, 0));
        let mut access = Access::new(&mut values, 2);

        assert_some_eq!(access.size_hint(), 2);
        assert_some_eq!(
            assert_ok!(access.next_element::<String>()),
            "foo".to_owned()
        );
        assert_some_eq!(access.size_hint(), 1);
        assert_some_eq!(assert_ok!(access.next_element::<u64>()), 42);
        assert_some_eq!(access.size_hint(), 0);
        assert_err_eq!(
            access.next_element::<bool>(),
            Error::new(error::Kind::EndOfValues, Position::new(0, 6))
        );
        assert_some_eq!(access.size_hint(), 0);
    }

    #[test]
    fn nested_values() {
        let mut values = Values::new(b"foo:42:1.2", Position::new(0, 0));
        let mut access = Access::new(&mut values, 3);

        assert_some_eq!(access.size_hint(), 3);
        assert_some_eq!(
            assert_ok!(access.next_element::<String>()),
            "foo".to_owned()
        );
        assert_some_eq!(access.size_hint(), 2);
        assert_some_eq!(assert_ok!(access.next_element::<(u64, ())>()), (42, ()));
        assert_some_eq!(access.size_hint(), 1);
        assert_some_eq!(assert_ok!(access.next_element::<f64>()), 1.2);
        assert_some_eq!(access.size_hint(), 0);
        assert_err_eq!(
            access.next_element::<bool>(),
            Error::new(error::Kind::EndOfValues, Position::new(0, 10))
        );
        assert_some_eq!(access.size_hint(), 0);
    }
}
