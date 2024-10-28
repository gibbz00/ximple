use std::{borrow::Cow, io::Write};

use crate::*;

pub trait ToXml {
    fn serialize(&self, serializer: &mut Serializer<impl Write>) -> Result<(), SerError>;

    /// Method indicating whether or not a serialization should be skipped,
    /// specifically the tags around the value
    ///
    /// Returns Some(fallback_value) if so.
    fn should_skip(&self) -> bool {
        false
    }

    /// Produce a list of attributes to serialize
    ///
    /// Attributes are usually part of the enclosing element and called before
    /// serializing the parent element start tag.
    fn attributes(&self) -> Attributes {
        Default::default()
    }
}

pub trait FromXml: Sized {
    fn deserialize(deserializer: &mut Deserializer<impl std::io::Read>) -> Result<Self, DeError>;

    /// Method indicating whether or not a deserialization is optional
    ///
    /// Returns Some(fallback_value) if so.
    fn fallback() -> Option<Self> {
        None
    }
}

/// To XML attribute value trait
pub trait ToXmlAttr {
    fn serialize(&self) -> Option<Cow<'_, str>>;
}
