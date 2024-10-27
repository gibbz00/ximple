use std::io::Write;

use crate::*;

pub struct Serializer<W> {
    event_writer: ::xml::EventWriter<W>,
}

impl<W: Write> Serializer<W> {
    pub(crate) fn new(pretty: bool, writer: W) -> Self {
        let event_writer = ::xml::EmitterConfig::new()
            .write_document_declaration(false)
            .perform_indent(pretty)
            .autopad_comments(pretty)
            .pad_self_closing(pretty)
            .create_writer(writer);

        Self { event_writer }
    }

    /// Used to serialize special elements provided by this crate.
    /// Ex. [`Document`](crate::provided::Document)
    pub(crate) fn event_writer(&mut self) -> &mut ::xml::EventWriter<W> {
        &mut self.event_writer
    }
}

impl<W: Write> Serializer<W> {
    /// Write a string value to the underlying writer
    ///
    /// All offending symbols, in particular, & and <, will be escaped.
    pub fn write_str(&mut self, str: &str) -> Result<(), SerError> {
        self.event_writer
            .write(::xml::writer::XmlEvent::characters(str))
            .map_err(Into::into)
    }

    // TODO: document;
    // how prefixes and namespaces are handled
    // wraps value in a name (give example)
    // "empty" values are autoclosed
    pub fn write_element<T: ToXml>(&mut self, name: &str, value: &T) -> Result<(), SerError> {
        if value.should_skip() {
            return Ok(());
        }

        self.write_start(name)?;
        value.serialize(self)?;
        self.write_end()
    }

    pub fn write_start(&mut self, name: &str) -> Result<(), SerError> {
        self.event_writer.write(::xml::writer::XmlEvent::start_element(name))?;
        Ok(())
    }

    pub fn write_end(&mut self) -> Result<(), SerError> {
        self.event_writer.write(::xml::writer::XmlEvent::end_element())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_element() {
        let str = "test";
        let xml = crate::test_utils::container::contained_xml("test");
        assert_serialize_str(&xml, &Container(str));
    }

    #[test]
    fn write_self_closing_element() {
        let xml = format!("<{}/>", crate::test_utils::container::ELEMENT_NAME);
        assert_serialize_str(&xml, &Container(()));
    }
}
