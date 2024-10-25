use std::io::Write;

use crate::*;

pub struct Serializer<W> {
    event_writer: ::xml::EventWriter<W>,
}

impl<W: Write> Serializer<W> {
    pub(crate) fn new(pretty: bool, writer: W) -> Self {
        let event_writer = ::xml::EmitterConfig::new()
            .perform_indent(pretty)
            .autopad_comments(pretty)
            .pad_self_closing(pretty)
            .write_document_declaration(false)
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
}
