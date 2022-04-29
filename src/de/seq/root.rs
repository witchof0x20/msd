use super::element;
use crate::de::{parse::Tags, Error, Result};
use serde::de::{DeserializeSeed, SeqAccess};
use std::io::Read;

pub(in crate::de) struct Access<'a, R> {
    tags: &'a mut Tags<R>,
}

impl<'a, R> Access<'a, R> {
    pub(in crate::de) fn new(tags: &'a mut Tags<R>) -> Self {
        Self { tags }
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
        let tag = match self.tags.next() {
            Ok(tag) => tag,
            Err(_) => return Ok(None),
        };
        let stored = tag.into_stored();
        unsafe { self.tags.revisit(stored) };
        Ok(Some(
            seed.deserialize(element::Deserializer::new(self.tags))?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::Access;
    use crate::de::parse::Tags;
    use claim::{assert_none, assert_ok, assert_some_eq};
    use serde::de::SeqAccess;

    #[test]
    fn next_element() {
        let mut tags = Tags::new(b"#foo;".as_slice());
        let mut access = Access::new(&mut tags);

        assert_some_eq!(assert_ok!(access.next_element::<String>()), "foo");
    }

    #[test]
    fn multiple_elements() {
        let mut tags = Tags::new(b"#foo;#bar;".as_slice());
        let mut access = Access::new(&mut tags);

        assert_some_eq!(assert_ok!(access.next_element::<String>()), "foo");
        assert_some_eq!(assert_ok!(access.next_element::<String>()), "bar");
    }

    #[test]
    fn next_element_none() {
        let mut tags = Tags::new(b"".as_slice());
        let mut access = Access::new(&mut tags);

        assert_none!(assert_ok!(access.next_element::<String>()));
    }
}
