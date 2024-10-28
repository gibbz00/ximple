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

impl<T: ToXmlAttr> ToXmlAttr for Option<T> {
    fn serialize(&self) -> Option<Cow<'_, str>> {
        self.as_ref().and_then(ToXmlAttr::serialize)
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

impl<T: ToXmlAttr> ToXmlAttr for Box<T> {
    fn serialize(&self) -> Option<Cow<'_, str>> {
        self.as_ref().serialize()
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

impl<T: ToXmlAttr + ToOwned + ?Sized> ToXmlAttr for Cow<'_, T> {
    fn serialize(&self) -> Option<Cow<'_, str>> {
        self.as_ref().serialize()
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
        assert_serialize_str(&xml, &Some(Container::new(())));
    }

    #[test]
    fn option_some_deserialization() {
        let xml = crate::test_utils::container::contained_xml("");
        assert_deserialize_str(&Some(Container::new(())), &xml);
    }

    #[test]
    fn option_none_serialization() {
        assert_serialize_str("", &Option::<Container<()>>::None);
    }

    #[test]
    fn option_none_deserialization() {
        assert_deserialize_str(&Container::new(Option::<Container<()>>::None), "<container></container>");
    }

    #[test]
    fn cow_serialization() {
        assert_serialize_str("test", &Cow::Borrowed("test"));
    }

    #[test]
    fn to_attr() {
        // TEMP: to be replace with bijictive assertion once `FromXmlAttr` is added

        let some_str = Some("test");
        assert_eq!(some_str.map(Cow::Borrowed), ToXmlAttr::serialize(&some_str));

        let none_str = Option::<&str>::None;
        assert_eq!(None, ToXmlAttr::serialize(&none_str));

        let boxed_str = Box::new("test");
        assert_eq!(Some(Cow::Borrowed(*boxed_str)), ToXmlAttr::serialize(&boxed_str));

        let cow_borrowed = Cow::Borrowed("test");
        assert_eq!(Some(Cow::Borrowed(&*cow_borrowed)), ToXmlAttr::serialize(&cow_borrowed));

        let cow_owned: Cow<'static, bool> = Cow::Owned(true);
        assert_eq!(Some(Cow::Borrowed("true")), ToXmlAttr::serialize(&cow_owned));
    }
}
