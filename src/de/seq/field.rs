use super::element;
use crate::de::{parse::Tags, Error, Result};
use serde::de::{DeserializeSeed, SeqAccess};
use std::io::Read;

pub(in crate::de) struct Access<'a, R> {
    field: &'static str,
    tags: &'a mut Tags<R>,
}

impl<'a, R> Access<'a, R> {
    pub(in crate::de) fn new(field: &'static str, tags: &'a mut Tags<R>) -> Self {
        Self { field, tags }
    }
}

impl<'a, 'de, R> SeqAccess<'de> for Access<'a, R>
where
    R: Read,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        let mut tag = match self.tags.next() {
            Ok(tag) => tag,
            Err(_) => return Ok(None),
        };

        // Check that the field name matches.
        let mut values = tag.next()?;
        let value = values.next()?;
        if value.parse_identifier()? == self.field {
            // Deserialize the rest of the tag.
            // SAFETY: `values` was created by a call to `tag.next()`.
            unsafe { tag.revisit(values) };
            let stored = tag.into_stored();
            unsafe { self.tags.revisit(stored) };
            Ok(Some(
                seed.deserialize(element::Deserializer::new(self.tags))?,
            ))
        } else {
            tag.reset();
            let stored = tag.into_stored();
            // SAFETY: `tag` points to the current buffer of `self.tags`.
            unsafe { self.tags.revisit(stored) };
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Access;
    use crate::de::parse::Tags;
    use claims::{assert_none, assert_ok, assert_some_eq};
    use serde::de::SeqAccess;
    use serde_derive::Deserialize;

    #[test]
    fn empty() {
        let mut tags = Tags::new(b"".as_slice());
        let mut access = Access::new("foo", &mut tags);

        assert_none!(assert_ok!(access.next_element::<()>()));
    }

    #[test]
    fn single() {
        let mut tags = Tags::new(b"#foo:42;\n".as_slice());
        let mut access = Access::new("foo", &mut tags);

        assert_some_eq!(assert_ok!(access.next_element::<u64>()), 42);
        assert_none!(assert_ok!(access.next_element::<u64>()));
    }

    #[test]
    fn multiple() {
        let mut tags = Tags::new(b"#foo:1;\n#foo:2;\n#foo:3;\n".as_slice());
        let mut access = Access::new("foo", &mut tags);

        assert_some_eq!(assert_ok!(access.next_element::<u64>()), 1);
        assert_some_eq!(assert_ok!(access.next_element::<u64>()), 2);
        assert_some_eq!(assert_ok!(access.next_element::<u64>()), 3);
        assert_none!(assert_ok!(access.next_element::<u64>()));
    }

    #[test]
    fn incorrect_field() {
        let mut tags = Tags::new(b"#bar:42;\n".as_slice());
        let mut access = Access::new("foo", &mut tags);

        assert_none!(assert_ok!(access.next_element::<u64>()));
    }

    #[test]
    fn multiple_structs() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Struct {
            bar: char,
            baz: u64,
        }
        let mut tags = Tags::new(
            b"#foo:;\n#bar:a;\n#baz:1;\n#foo:;\n#bar:b;\n#baz:2;\n#foo:;\n#bar:c;\n#baz:3;\n"
                .as_slice(),
        );
        let mut access = Access::new("foo", &mut tags);

        assert_some_eq!(
            assert_ok!(access.next_element::<Struct>()),
            Struct { bar: 'a', baz: 1 }
        );
        assert_some_eq!(
            assert_ok!(access.next_element::<Struct>()),
            Struct { bar: 'b', baz: 2 }
        );
        assert_some_eq!(
            assert_ok!(access.next_element::<Struct>()),
            Struct { bar: 'c', baz: 3 }
        );
        assert_none!(assert_ok!(access.next_element::<Struct>()));
    }
}
