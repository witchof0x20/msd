mod field;
mod value;

use crate::de::{
    parse::{StoredTag, StoredValues, Tags},
    Error, Result,
};
use serde::de::{DeserializeSeed, MapAccess};
use std::{collections::HashSet, io::Read};

pub(in crate::de) struct Access<'a, R> {
    tags: &'a mut Tags<R>,
    fields: HashSet<&'static str>,

    // These stored fields contain raw pointers to the internal buffers of the tag and values
    // respectively. Note that the pointed-to buffers are only guaranteed to be valid until another
    // call to `self.tags.next()`.
    tag: Option<StoredTag>,
    values: Option<StoredValues>,
    field: Option<&'static str>,
}

impl<'a, R> Access<'a, R> {
    pub(in crate::de) fn new(tags: &'a mut Tags<R>, fields: &'static [&'static str]) -> Self {
        Self {
            tags,
            fields: fields.into_iter().copied().collect(),

            tag: None,
            values: None,
            field: None,
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
        let field = values.next()?.parse_identifier()?;

        // Only return the result if the field is in the list of possible fields for the struct.
        if let Some(static_field) = self.fields.take(field.as_str()) {
            let result = seed.deserialize(field::Deserializer::new(&field))?;
            // Note that these raw values will only live until the next call to `next_key_seed()`, at
            // which point they will be overwritten.
            self.values = Some(values.into_stored());
            self.tag = Some(tag.into_stored());
            self.field = Some(static_field);
            Ok(Some(result))
        } else {
            tag.reset();
            let stored_tag = tag.into_stored();
            // SAFETY: `stored_tag` references the buffer still active in `self.tags`.
            unsafe { self.tags.revisit(stored_tag) };
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        // SAFETY: `self.tags` is not modified here, so this `Tag` will live longer than the
        // referenced buffer.
        let tag = self
            .tag
            .take()
            .expect("call to `next_value()` not preceeded by successful call to `next_key()`");
        let values = self
            .values
            .take()
            .expect("call to `next_value()` not preceeded by successful call to `next_key()`");
        let field = self
            .field
            .take()
            .expect("call to `next_value()` not preceeded by successful call to `next_key()`");

        seed.deserialize(value::Deserializer::new(field, self.tags, tag, values))
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
        let mut tag = match self.tags.next() {
            Ok(tag) => tag,
            Err(_) => return Ok(None),
        };
        let mut values = tag.next()?;
        let field = values.next()?.parse_identifier()?;

        // Only return the result if the field is in the list of possible fields for the struct.
        if let Some(static_field) = self.fields.take(field.as_str()) {
            let key = key_seed.deserialize(field::Deserializer::new(&field))?;
            let stored_tag = tag.into_stored();
            let stored_values = values.into_stored();
            let value = value_seed.deserialize(value::Deserializer::new(
                static_field,
                self.tags,
                stored_tag,
                stored_values,
            ))?;
            Ok(Some((key, value)))
        } else {
            tag.reset();
            let stored_tag = tag.into_stored();
            // SAFETY: `stored_tag` references the buffer still active in `self.tags`.
            unsafe { self.tags.revisit(stored_tag) };
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Access;
    use crate::de::parse::{Tag, Tags};
    use claim::{assert_none, assert_ok, assert_ok_eq, assert_some_eq};
    use serde::{
        de,
        de::{MapAccess, Visitor},
        Deserialize,
    };
    use std::fmt;

    #[derive(Debug, PartialEq)]
    struct Identifier(String);

    impl<'de> Deserialize<'de> for Identifier {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct IdentifierVisitor;

            impl<'de> Visitor<'de> for IdentifierVisitor {
                type Value = Identifier;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("identifier")
                }

                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Ok(Identifier(value.to_owned()))
                }
            }

            deserializer.deserialize_identifier(IdentifierVisitor)
        }
    }

    #[test]
    fn next_key_and_value() {
        let mut tags = Tags::new(b"#foo:42;\n".as_slice());
        let mut access = Access::new(&mut tags, &["foo"]);

        assert_some_eq!(
            assert_ok!(access.next_key::<Identifier>()),
            Identifier("foo".to_owned())
        );
        assert_ok_eq!(access.next_value::<u64>(), 42);
    }

    #[test]
    #[should_panic]
    fn next_value_without_next_key() {
        // Should panic if `next_value()` is called before `next_key()`.
        let mut tags = Tags::new(b"#42;\n".as_slice());
        let mut access = Access::new(&mut tags, &["foo"]);

        let _ = access.next_value::<u64>();
    }

    #[test]
    fn next_key_none() {
        let mut tags = Tags::new(b"".as_slice());
        let mut access = Access::new(&mut tags, &["foo"]);

        assert_none!(assert_ok!(access.next_key::<Identifier>()));
    }

    #[test]
    fn next_key_not_in_field_list() {
        let mut tags = Tags::new(b"#bar:42;\n".as_slice());
        let mut access = Access::new(&mut tags, &["foo"]);

        assert_none!(assert_ok!(access.next_key::<Identifier>()));

        // Should also revisit the tag.
        assert_ok_eq!(tags.next(), Tag::new(b"bar:42;\n", 0, 0));
    }

    #[test]
    #[should_panic]
    fn next_value_after_next_key_none() {
        // Should panic if `next_value()` is called before `next_key()`.
        let mut tags = Tags::new(b"".as_slice());
        let mut access = Access::new(&mut tags, &["foo"]);

        assert_none!(assert_ok!(access.next_key::<Identifier>()));
        let _ = access.next_value::<u64>();
    }

    #[test]
    fn next_entry() {
        let mut tags = Tags::new(b"#foo:42;\n".as_slice());
        let mut access = Access::new(&mut tags, &["foo"]);

        assert_some_eq!(
            assert_ok!(access.next_entry::<Identifier, u64>()),
            (Identifier("foo".to_owned()), 42)
        );
    }

    #[test]
    fn next_entry_none() {
        let mut tags = Tags::new(b"".as_slice());
        let mut access = Access::new(&mut tags, &["foo"]);

        assert_none!(assert_ok!(access.next_entry::<Identifier, u64>()));
    }

    #[test]
    fn next_entry_not_in_field_list() {
        let mut tags = Tags::new(b"#bar:42;\n".as_slice());
        let mut access = Access::new(&mut tags, &["foo"]);

        assert_none!(assert_ok!(access.next_entry::<Identifier, u64>()));

        // Should also revisit the tag.
        assert_ok_eq!(tags.next(), Tag::new(b"bar:42;\n", 0, 0));
    }
}
