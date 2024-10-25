use std::io::Write;

use crate::*;

pub trait ToXml {
    fn serialize(&self, serializer: &mut Serializer<impl Write>) -> Result<(), SerError>;
}
