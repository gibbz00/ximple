use crate::*;

impl ToXml for str {
    fn serialize(&self, serializer: &mut Serializer<impl std::io::Write>) -> Result<(), SerError> {
        serializer.write_str(self)
    }
}

impl<T: ToXml + ?Sized> ToXml for &T {
    fn serialize(&self, serializer: &mut Serializer<impl std::io::Write>) -> Result<(), SerError> {
        (*self).serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_serialization() {
        assert_serialize_str("test", &"test");
    }
}
