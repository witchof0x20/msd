use crate::de::{
    parse::{Tag, Values},
    tuple, Error, Result,
};
use serde::de::{DeserializeSeed, MapAccess};

pub(in crate::de) struct Access<'a, 'b> {
    tag: &'a mut Tag<'b>,

    values: Option<Values<'a>>,
}

impl<'a, 'b> Access<'a, 'b> {
    pub(in crate::de) fn new(tag: &'a mut Tag<'b>) -> Self {
        Self { tag, values: None }
    }
}

impl<'a, 'b, 'de> MapAccess<'de> for Access<'a, 'b> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        let mut values = match self.tag.next() {
            Ok(values) => values,
            Err(_) => return Ok(None),
        };
        let key = seed.deserialize(tuple::element::Deserializer::new(&mut values))?;
        self.values = Some(values);

        Ok(Some(key))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        let mut values = self
            .values
            .take()
            .expect("call to `next_value()` not preceeded by successful call to `next_key()`");

        let value = seed.deserialize(tuple::element::Deserializer::new(&mut values))?;
        values.assert_exhausted()?;

        Ok(value)
    }

    fn next_entry_seed<K, V>(
        &mut self,
        key_seed: K,
        value_seed: V,
    ) -> Result<Option<(K::Value, V::Value)>>
    where
        K: DeserializeSeed<'de>,
        V: DeserializeSeed<'de>,
    {
        let mut values = match self.tag.next() {
            Ok(values) => values,
            Err(_) => return Ok(None),
        };
        let key = key_seed.deserialize(tuple::element::Deserializer::new(&mut values))?;

        let value = value_seed.deserialize(tuple::element::Deserializer::new(&mut values))?;
        values.assert_exhausted()?;

        Ok(Some((key, value)))
    }
}

#[cfg(test)]
mod tests {
    use super::Access;
    use crate::de::{parse::Tag, Position};
    use claim::{assert_none, assert_ok, assert_ok_eq, assert_some_eq};
    use serde::de::MapAccess;

    #[test]
    fn next_key_and_value() {
        let mut tag = Tag::new(b"foo:42;", Position::new(0, 0));
        let mut access = Access::new(&mut tag);

        assert_some_eq!(assert_ok!(access.next_key::<String>()), "foo".to_owned());
        assert_ok_eq!(access.next_value::<u64>(), 42);
    }

    #[test]
    fn multiple_keys_and_values() {
        let mut tag = Tag::new(b"foo:42;bar:100", Position::new(0, 0));
        let mut access = Access::new(&mut tag);

        assert_some_eq!(assert_ok!(access.next_key::<String>()), "foo".to_owned());
        assert_ok_eq!(access.next_value::<u64>(), 42);
        assert_some_eq!(assert_ok!(access.next_key::<String>()), "bar".to_owned());
        assert_ok_eq!(access.next_value::<u64>(), 100);
    }

    #[test]
    #[should_panic]
    fn next_value_without_next_key() {
        // Should panic if `next_value()` is called before `next_key()`.
        let mut tag = Tag::new(b"42;", Position::new(0, 0));
        let mut access = Access::new(&mut tag);

        let _ = access.next_value::<u64>();
    }

    #[test]
    fn next_key_none() {
        let mut tag = Tag::new(b"", Position::new(0, 0));
        assert_ok!(tag.next());
        let mut access = Access::new(&mut tag);

        assert_none!(assert_ok!(access.next_key::<String>()));
    }

    #[test]
    fn next_entry() {
        let mut tag = Tag::new(b"foo:42;", Position::new(0, 0));
        let mut access = Access::new(&mut tag);

        assert_some_eq!(
            assert_ok!(access.next_entry::<String, u64>()),
            ("foo".to_owned(), 42)
        );
    }

    #[test]
    fn multiple_entries() {
        let mut tag = Tag::new(b"foo:42;bar:100;", Position::new(0, 0));
        let mut access = Access::new(&mut tag);

        assert_some_eq!(
            assert_ok!(access.next_entry::<String, u64>()),
            ("foo".to_owned(), 42)
        );
        assert_some_eq!(
            assert_ok!(access.next_entry::<String, u64>()),
            ("bar".to_owned(), 100)
        );
    }

    #[test]
    fn next_entry_none() {
        let mut tag = Tag::new(b"", Position::new(0, 0));
        assert_ok!(tag.next());
        let mut access = Access::new(&mut tag);

        assert_none!(assert_ok!(access.next_entry::<String, u64>()));
    }
}
