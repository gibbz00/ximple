use std::io::Write;

pub struct Serializer<W> {
    event_writer: xml::EventWriter<W>,
}

impl<W: Write> Serializer<W> {
    pub(crate) fn new(pretty: bool, writer: W) -> Self {
        let event_writer = xml::EmitterConfig::new()
            .perform_indent(pretty)
            .autopad_comments(pretty)
            .pad_self_closing(pretty)
            .write_document_declaration(false)
            .create_writer(writer);

        Self { event_writer }
    }

    /// Used to serialize special elements provided by this crate.
    /// Ex. [`Document`](crate::provided::Document)
    pub(crate) fn event_writer(&mut self) -> &mut xml::EventWriter<W> {
        &mut self.event_writer
    }
}
