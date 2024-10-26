use std::io::Read;

use crate::*;

pub struct Deserializer<R: Read> {
    stream: EventStream<R>,
}

impl<R: Read> Deserializer<R> {
    pub(crate) fn new(reader: R) -> Self {
        Self { stream: EventStream::new(reader) }
    }

    /// Used to deserialize special elements provided by this crate.
    /// Ex. [`Document`](crate::provided::Document)
    pub(crate) fn event_stream(&mut self) -> &mut EventStream<R> {
        &mut self.stream
    }
}

impl<R: Read> Deserializer<R> {
    pub fn read_event(&mut self) -> Result<Event, DeError> {
        let event_reader = self.event_stream();

        while let Some(xml_event) = event_reader.next().transpose().map_err(DeError::from_reader)? {
            if let Some(event) = Event::from_xml_event(xml_event) {
                return Ok(event);
            }
        }

        Err(DeError::end())
    }

    pub fn read_string(&mut self) -> Result<String, DeError> {
        let found_event = self.read_event()?;

        let Event::Characters(string) = found_event else {
            return Err(DeError::invalid_event(EventType::Characters, found_event));
        };

        Ok(string)
    }

    /// Deserializes a value within an open and close tag
    ///
    /// Enclosing tags must match the supplied name.
    pub fn read_element<T: FromXml>(&mut self, name: Name<'_>) -> Result<T, DeError> {
        self.read_start_element(name)?;
        let value = T::deserialize(self)?;
        self.read_end_element(name)?;
        Ok(value)
    }

    /// Read a start tag that must match the supplied name.
    fn read_start_element(&mut self, name: Name<'_>) -> Result<(), DeError> {
        match self.read_event()? {
            Event::StartElement(start_element) => {
                name.matches(start_element.name.borrow())
                    .map_err(|err| DeError::from_inner(InnerError::InvalidName(Tag::Start, err)))?;

                // TODO: place attributes in deserializer context
                Ok(())
            }
            other => Err(DeError::invalid_event(EventType::StartElement, other)),
        }
    }

    /// Read an end tag that must match the supplied name.
    fn read_end_element(&mut self, name: Name<'_>) -> Result<(), DeError> {
        match self.read_event()? {
            Event::EndElement(end_element) => name
                .matches(end_element.borrow())
                .map_err(|err| DeError::from_inner(InnerError::InvalidName(Tag::End, err))),
            other => Err(DeError::invalid_event(EventType::StartElement, other)),
        }
    }
}
