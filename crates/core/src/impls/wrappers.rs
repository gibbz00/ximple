use std::borrow::Cow;

use crate::*;

impl<T: ToXml> ToXml for Option<T> {
    fn serialize(&self, serializer: &mut Serializer<impl std::io::Write>) -> Result<(), SerError> {
        if let Some(target) = self.as_ref() {
            target.serialize(serializer)?;
        }

        Ok(())
    }
}

impl<T: ToXml> ToXml for Box<T> {
    fn serialize(&self, serializer: &mut Serializer<impl std::io::Write>) -> Result<(), SerError> {
        self.as_ref().serialize(serializer)
    }
}

impl<T: ToXml + ToOwned + ?Sized> ToXml for Cow<'_, T> {
    fn serialize(&self, serializer: &mut Serializer<impl std::io::Write>) -> Result<(), SerError> {
        self.as_ref().serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_none_serialization() {
        assert_serialize_str("", &Option::<&str>::None);
    }

    #[test]
    fn option_some_serialization() {
        assert_serialize_str("test", &Some("test"));
    }

    #[test]
    fn box_serialization() {
        assert_serialize_str("test", &Box::new("test"));
    }

    #[test]
    fn cow_serialization() {
        assert_serialize_str("test", &Cow::Borrowed("test"));
        assert_serialize_str("test", &Cow::<'_, str>::Owned("test".to_string()));
    }
}
