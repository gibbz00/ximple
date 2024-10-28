use std::{
    borrow::Cow,
    io::{Read, Write},
};

use crate::*;

impl ToXml for String {
    fn serialize(&self, serializer: &mut Serializer<impl Write>) -> Result<(), SerError> {
        serializer.write_str(self)
    }
}

impl ToXmlAttr for String {
    fn serialize(&self) -> Option<Cow<'_, str>> {
        Some(Cow::Borrowed(self))
    }
}

impl FromXml for String {
    fn deserialize(deserializer: &mut Deserializer<impl Read>) -> Result<Self, DeError> {
        deserializer.read_string()
    }
}

impl<T: ToXml> ToXml for Vec<T> {
    fn serialize(&self, serializer: &mut Serializer<impl Write>) -> Result<(), SerError> {
        self.iter().try_for_each(|element| element.serialize(serializer))
    }
}

impl<T: FromXml> FromXml for Vec<T> {
    fn deserialize(deserializer: &mut Deserializer<impl Read>) -> Result<Self, DeError> {
        let mut buffer = Vec::new();
        let start_depth = deserializer.event_stream().depth();

        loop {
            if deserializer.event_stream().peek_depth() < start_depth {
                break;
            }

            if let Some(Ok(peeked_event)) = deserializer.event_stream().peek() {
                if peeked_event == &::xml::reader::XmlEvent::EndDocument {
                    break;
                }
            }

            let value = T::deserialize(deserializer)?;
            buffer.push(value);
        }

        Ok(buffer)
    }
}

impl<T: ToXml> ToXml for [T] {
    fn serialize(&self, serializer: &mut Serializer<impl Write>) -> Result<(), SerError> {
        self.iter().try_for_each(|element| element.serialize(serializer))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    assert_bijective_xml!(string, "test", "test".to_string());

    mod vec {
        use super::*;

        const VEC_ELEMENTS: [&str; 4] = ["t", "e", "s", "t"];

        fn mock_vec() -> Vec<Container<'static, String>> {
            VEC_ELEMENTS.iter().map(|value| Container::new(value.to_string())).collect()
        }

        fn mock_vec_xml() -> String {
            VEC_ELEMENTS
                .iter()
                .map(|value| crate::test_utils::container::contained_xml(value))
                .fold(String::new(), |mut acc, xml_piece| {
                    acc.push_str(&xml_piece);
                    acc
                })
        }

        assert_bijective_xml!(&mock_vec_xml(), mock_vec());
    }

    #[test]
    fn slice_serialization() {
        assert_serialize_str("test", &["t", "e", "s", "t"].as_slice());
    }

    #[test]
    fn to_attr() {
        // TEMP: to be replace with bijictive assertion once `FromXmlAttr` is added

        let string = "test".to_string();
        assert_eq!(Some(Cow::Borrowed(string.as_str())), ToXmlAttr::serialize(&string));
    }
}
