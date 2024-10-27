use crate::*;

pub trait FromXml: Sized {
    fn deserialize(deserializer: &mut Deserializer<impl std::io::Read>) -> Result<Self, DeError>;

    /// Method indicating whether or not a deserialization is optional
    ///
    /// Returns Some(fallback_value) if so.
    fn fallback() -> Option<Self> {
        None
    }
}
