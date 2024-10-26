use crate::*;

pub trait FromXml: Sized {
    fn deserialize(deserializer: &mut Deserializer<impl std::io::Read>) -> Result<Self, DeError>;
}
