use crate::*;

impl ToXml for String {
    fn serialize(&self, serializer: &mut Serializer<impl std::io::Write>) -> Result<(), SerError> {
        serializer.write_str(self)
    }
}

impl<T: ToXml> ToXml for Vec<T> {
    fn serialize(&self, serializer: &mut Serializer<impl std::io::Write>) -> Result<(), SerError> {
        self.iter().try_for_each(|element| element.serialize(serializer))
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

    #[test]
    fn string_serialization() {
        assert_serialize_str("test", &"test".to_string());
    }

    #[test]
    fn vec_serialization() {
        assert_serialize_str("test", &vec!["t", "e", "s", "t"]);
    }

    #[test]
    fn slice_serialization() {
        assert_serialize_str("test", &["t", "e", "s", "t"].as_slice());
    }
}
