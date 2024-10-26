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

impl FromXml for bool {
    fn deserialize(deserializer: &mut Deserializer<impl std::io::Read>) -> Result<Self, DeError> {
        let string = deserializer.read_string()?;

        match string.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(DeError::invalid_value("true or false", string)),
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

    assert_bijective_xml!(bool_true, "true", true);
    assert_bijective_xml!(bool_false, "false", false);
}
