use std::io::Write;

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
}
