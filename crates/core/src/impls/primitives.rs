use crate::*;

impl ToXml for str {
    fn serialize(&self, serializer: &mut Serializer<impl std::io::Write>) -> Result<(), SerError> {
        serializer.write_str(self)
    }
}

impl ToXml for bool {
    fn serialize(&self, serializer: &mut Serializer<impl std::io::Write>) -> Result<(), SerError> {
        match self {
            true => serializer.write_str("true"),
            false => serializer.write_str("false"),
        }
    }
}

impl<T: ToXml + ?Sized> ToXml for &T {
    fn serialize(&self, serializer: &mut Serializer<impl std::io::Write>) -> Result<(), SerError> {
        (*self).serialize(serializer)
    }
}

// TODO: document element should be autoclosed
impl ToXml for () {
    fn serialize(&self, _: &mut Serializer<impl std::io::Write>) -> Result<(), SerError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_serialization() {
        assert_serialize_str("test", &"test");
    }

    #[test]
    fn bool_serialization() {
        assert_serialize_str("true", &true);
        assert_serialize_str("false", &false);
    }
}
