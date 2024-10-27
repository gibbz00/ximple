use std::{
    borrow::Cow,
    io::{Read, Write},
};

use crate::*;

impl<T: ToXml> ToXml for Option<T> {
    fn serialize(&self, serializer: &mut Serializer<impl Write>) -> Result<(), SerError> {
        if let Some(target) = self.as_ref() {
            target.serialize(serializer)?;
        }

        Ok(())
    }

    fn should_skip(&self) -> bool {
        self.is_none()
    }
}

impl<T: FromXml> FromXml for Option<T> {
    fn deserialize(deserializer: &mut Deserializer<impl Read>) -> Result<Self, DeError> {
        // Deserializer::read_start_element determines if the enclosing tags for
        // this value should be deserialized, before this method is called
        T::deserialize(deserializer).map(Some)
    }

    fn fallback() -> Option<Self> {
        Some(None)
    }
}

impl<T: ToXml> ToXml for Box<T> {
    fn serialize(&self, serializer: &mut Serializer<impl Write>) -> Result<(), SerError> {
        self.as_ref().serialize(serializer)
    }
}

impl<T: FromXml> FromXml for Box<T> {
    fn deserialize(deserializer: &mut Deserializer<impl Read>) -> Result<Self, DeError> {
        T::deserialize(deserializer).map(Box::new)
    }
}

impl<T: ToXml + ToOwned + ?Sized> ToXml for Cow<'_, T> {
    fn serialize(&self, serializer: &mut Serializer<impl Write>) -> Result<(), SerError> {
        self.as_ref().serialize(serializer)
    }
}

impl<T: ToOwned + ?Sized> FromXml for Cow<'_, T>
where
    T::Owned: FromXml,
{
    fn deserialize(deserializer: &mut Deserializer<impl Read>) -> Result<Self, DeError> {
        <T::Owned>::deserialize(deserializer).map(Cow::Owned)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    assert_bijective_xml!(boxed, "test", Box::new("test".to_string()));
    assert_bijective_xml!(cow_owned, "test", Cow::<'_, str>::Owned("test".to_string()));

    #[test]
    fn option_some_serialization() {
        let xml = format!("<{}/>", crate::test_utils::container::ELEMENT_NAME);
        assert_serialize_str(&xml, &Some(Container(())));
    }

    #[test]
    fn option_some_deserialization() {
        let xml = crate::test_utils::container::contained_xml("");
        assert_deserialize_str(&Some(Container(())), &xml);
    }

    #[test]
    fn option_none_serialization() {
        assert_serialize_str("", &Option::<Container<()>>::None);
    }

    #[test]
    fn option_none_deserialization() {
        assert_deserialize_str(&Container(Option::<Container<()>>::None), "<container></container>");
    }

    #[test]
    fn cow_serialization() {
        assert_serialize_str("test", &Cow::Borrowed("test"));
    }
}
