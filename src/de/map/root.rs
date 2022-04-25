use crate::de::{
    parse::{StoredValues, Tags},
    Error, Result, tuple,
};
use serde::de::{DeserializeSeed, MapAccess};
use std::io::Read;

pub(in crate::de) struct Access<'a, R> {
    tags: &'a mut Tags<R>,

    values: Option<StoredValues>,
}

impl<'a, R> Access<'a, R> {
    pub(in crate::de) fn new(tags: &'a mut Tags<R>) -> Self {
        Self {
            tags,

            values: None,
        }
    }
}

impl<'a, 'de, R> MapAccess<'de> for Access<'a, R>
where
    R: Read,
{
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        let mut tag = match self.tags.next() {
            Ok(tag) => tag,
            Err(_) => return Ok(None),
        };
        let mut values = tag.next()?;
        tag.assert_exhausted()?;
        let key = seed.deserialize(tuple::element::Deserializer::new(&mut values))?;
        self.values = Some(values.into_stored());

        Ok(Some(key))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        // SAFETY: `self.tags` is not modified here, so this `Values` will live longer than the
        // referenced buffer.
        let mut values = unsafe {
            self.values
                .take()
                .expect("call to `next_value()` not preceeded by successful call to `next_key()`")
                .into_values()
        };

        let value = seed.deserialize(tuple::element::Deserializer::new(&mut values))?;
        values.assert_exhausted()?;

        Ok(value)
    }

    fn next_entry_seed<K, V>(&mut self, key_seed: K, value_seed: V) -> Result<Option<(K::Value, V::Value)>> where K: DeserializeSeed<'de>, V: DeserializeSeed<'de> {
        let mut tag = match self.tags.next() {
            Ok(tag) => tag,
            Err(_) => return Ok(None),
        };
        let mut values = tag.next()?;
        tag.assert_exhausted()?;
        let key = key_seed.deserialize(tuple::element::Deserializer::new(&mut values))?;

        let value = value_seed.deserialize(tuple::element::Deserializer::new(&mut values))?;
        values.assert_exhausted()?;

        Ok(Some((key, value)))
    }
}

#[cfg(test)]
mod tests {
    use super::Access;
    use crate::de::parse::Tags;
    use claim::{assert_none, assert_ok, assert_ok_eq, assert_some_eq};
    use serde::de::MapAccess;

    #[test]
    fn next_key_and_value() {
        let mut tags = Tags::new(b"#foo:42;\n".as_slice());
        let mut access = Access::new(&mut tags);

        assert_some_eq!(assert_ok!(access.next_key::<String>()), "foo".to_owned());
        assert_ok_eq!(access.next_value::<u64>(), 42);
    }

    #[test]
    #[should_panic]
    fn next_value_without_next_key() {
        // Should panic if `next_value()` is called before `next_key()`.
        let mut tags = Tags::new(b"#42;\n".as_slice());
        let mut access = Access::new(&mut tags);

        let _ = access.next_value::<u64>();
    }

    #[test]
    fn next_key_none() {
        let mut tags = Tags::new(b"".as_slice());
        let mut access = Access::new(&mut tags);

        assert_none!(assert_ok!(access.next_key::<String>()));
    }

    #[test]
    fn next_entry() {
        let mut tags = Tags::new(b"#foo:42;\n".as_slice());
        let mut access = Access::new(&mut tags);

        assert_some_eq!(assert_ok!(access.next_entry::<String, u64>()), ("foo".to_owned(), 42));
    }

    #[test]
    fn next_entry_none() {
        let mut tags = Tags::new(b"".as_slice());
        let mut access = Access::new(&mut tags);

        assert_none!(assert_ok!(access.next_entry::<String, u64>()));
    }
}
