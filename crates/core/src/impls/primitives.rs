use std::{
    borrow::Cow,
    io::{Read, Write},
};

use crate::*;

impl ToXml for str {
    fn serialize(&self, serializer: &mut Serializer<impl Write>) -> Result<(), SerError> {
        serializer.write_str(self)
    }
}

impl ToXmlAttr for str {
    fn serialize(&self) -> Option<std::borrow::Cow<'_, str>> {
        Some(Cow::Borrowed(self))
    }
}

impl ToXml for bool {
    fn serialize(&self, serializer: &mut Serializer<impl Write>) -> Result<(), SerError> {
        match self {
            true => serializer.write_str("true"),
            false => serializer.write_str("false"),
        }
    }
}

impl ToXmlAttr for bool {
    fn serialize(&self) -> Option<Cow<'_, str>> {
        Some(Cow::Borrowed(match self {
            true => "true",
            false => "false",
        }))
    }
}

impl FromXml for bool {
    fn deserialize(deserializer: &mut Deserializer<impl Read>) -> Result<Self, DeError> {
        let string = deserializer.read_string()?;

        match string.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(DeError::invalid_value("true or false", string)),
        }
    }
}

impl<T: ToXml + ?Sized> ToXml for &T {
    fn serialize(&self, serializer: &mut Serializer<impl Write>) -> Result<(), SerError> {
        (*self).serialize(serializer)
    }
}

impl<T: ToXmlAttr + ?Sized> ToXmlAttr for &T {
    fn serialize(&self) -> Option<Cow<'_, str>> {
        (*self).serialize()
    }
}

// TODO: enclosing document element should be autoclosed
impl ToXml for () {
    fn serialize(&self, _: &mut Serializer<impl Write>) -> Result<(), SerError> {
        Ok(())
    }
}

impl FromXml for () {
    fn deserialize(_: &mut Deserializer<impl Read>) -> Result<Self, DeError> {
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

    #[test]
    fn to_attr() {
        // TEMP: to be replace with bijictive assertion once `FromXmlAttr` is added

        let str = "test";
        assert_eq!(Some(Cow::Borrowed(str)), ToXmlAttr::serialize(str));

        let bool_true = true;
        assert_eq!(Some(Cow::Borrowed("true")), ToXmlAttr::serialize(&bool_true));

        let bool_false = false;
        assert_eq!(Some(Cow::Borrowed("false")), ToXmlAttr::serialize(&bool_false));
    }
}
