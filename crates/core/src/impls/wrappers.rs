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
}

impl<T: FromXml> FromXml for Option<T> {
    fn deserialize(deserializer: &mut Deserializer<impl Read>) -> Result<Self, DeError> {
        if deserializer.event_stream().peek_depth() < deserializer.event_stream().depth() {
            return Ok(None);
        }

        if let Some(Ok(peeked_event)) = deserializer.event_stream().peek() {
            if peeked_event == &::xml::reader::XmlEvent::EndDocument {
                return Ok(None);
            }
        }

        T::deserialize(deserializer).map(Some)
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

    assert_bijective_xml!(option_none, "", Option::<bool>::None);
    assert_bijective_xml!(option_some, "test", Some("test".to_string()));
    assert_bijective_xml!(boxed, "test", Box::new("test".to_string()));
    assert_bijective_xml!(cow_owned, "test", Cow::<'_, str>::Owned("test".to_string()));

    #[test]
    fn cow_serialization() {
        assert_serialize_str("test", &Cow::Borrowed("test"));
    }
}
