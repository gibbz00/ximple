use crate::*;

impl ToXml for String {
    fn serialize(&self, serializer: &mut Serializer<impl std::io::Write>) -> Result<(), SerError> {
        serializer.write_str(self)
    }
}

impl FromXml for String {
    fn deserialize(deserializer: &mut Deserializer<impl std::io::Read>) -> Result<Self, DeError> {
        deserializer.read_string()
    }
}

impl<T: ToXml> ToXml for Vec<T> {
    fn serialize(&self, serializer: &mut Serializer<impl std::io::Write>) -> Result<(), SerError> {
        self.iter().try_for_each(|element| element.serialize(serializer))
    }
}

impl<T: FromXml> FromXml for Vec<T> {
    fn deserialize(deserializer: &mut Deserializer<impl std::io::Read>) -> Result<Self, DeError> {
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
    fn serialize(&self, serializer: &mut Serializer<impl std::io::Write>) -> Result<(), SerError> {
        self.iter().try_for_each(|element| element.serialize(serializer))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    assert_bijective_xml!(string, "test", "test".to_string());

    #[test]
    fn vec_serialization() {
        assert_serialize_str("test", &vec!["t", "e", "s", "t"]);
    }

    #[test]
    fn vec_deserialization() {
        let slice = ["t", "e", "s", "t"];
        let vec = slice.iter().map(|value| Container(value.to_string())).collect::<Vec<_>>();
        let xml = slice
            .iter()
            .map(|value| crate::test_utils::container::contained_xml(value))
            .fold(String::new(), |mut acc, xml_piece| {
                acc.push_str(&xml_piece);
                acc
            });

        assert_deserialize_str(&vec, &xml);
    }

    #[test]
    fn slice_serialization() {
        assert_serialize_str("test", &["t", "e", "s", "t"].as_slice());
    }
}
