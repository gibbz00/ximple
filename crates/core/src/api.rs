// TODO: document with examples

mod read {
    use std::io::Read;

    use crate::*;

    pub fn from_str<T: FromXml>(str: &str) -> Result<T, DeError> {
        from_reader(str.as_bytes())
    }

    pub fn from_reader<T: FromXml>(reader: impl Read) -> Result<T, DeError> {
        let mut deserializer = Deserializer::new(reader);
        T::deserialize(&mut deserializer)
    }
}
pub use read::{from_reader, from_str};

mod write {
    use std::io::Write;

    use crate::*;

    pub fn to_string<T: ToXml>(value: &T) -> Result<String, SerError> {
        to_string_impl(false, value)
    }

    pub fn to_string_pretty<T: ToXml>(value: &T) -> Result<String, SerError> {
        to_string_impl(true, value)
    }

    fn to_string_impl<T: ToXml>(pretty: bool, value: &T) -> Result<String, SerError> {
        let mut buffer = Vec::new();
        to_writer_impl(pretty, &mut buffer, value)?;

        let string = String::from_utf8(buffer).expect("invalid UTF-8 received from XML writer");
        Ok(string)
    }

    /// Simple writer optimized for compact XML output
    pub fn to_writer<T: ToXml>(writer: impl Write, value: &T) -> Result<(), SerError> {
        to_writer_impl(false, writer, value)
    }

    /// Pretty writer optimized for human readable XML output
    ///
    /// Adds indentation together with comment and self-closing tag padding.
    pub fn to_writer_pretty<T: ToXml>(writer: impl Write, value: &T) -> Result<(), SerError> {
        to_writer_impl(true, writer, value)
    }

    fn to_writer_impl<T: ToXml>(pretty: bool, writer: impl Write, value: &T) -> Result<(), SerError> {
        let mut serializer = Serializer::new(pretty, writer);
        value.serialize(&mut serializer)?;

        Ok(())
    }
}
pub use write::{to_string, to_string_pretty, to_writer, to_writer_pretty};
