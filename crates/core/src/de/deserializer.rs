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
        if let Some(fallback) = self.read_start_element::<T>(name)? {
            return Ok(fallback);
        }
        let value = T::deserialize(self)?;
        self.read_end_element(name)?;
        Ok(value)
    }

    /// Read a start tag that must match the supplied name.
    ///
    /// Advances the read cursor and is therefore not a idempotent operation,
    /// unless the next element is not found and T has a fallback.
    ///
    /// Returns Some(fallback_value) if the element wasn't found, and where `T` is marked as
    /// optional by [`FromXml::fallback`]. Returns element not found error if
    pub fn read_start_element<T: FromXml>(&mut self, name: Name<'_>) -> Result<Option<T>, DeError> {
        if !self.peek_start_element_matches(name) {
            if let Some(fallback) = T::fallback() {
                return Ok(Some(fallback));
            }
        }

        match self.read_event()? {
            Event::StartElement(start_element) => {
                name.matches(start_element.name.borrow())
                    .map_err(|err| DeError::from_inner(InnerError::InvalidName(Tag::Start, err)))?;

                // TODO: place attributes in deserializer context
                Ok(None)
            }
            other => Err(DeError::invalid_event(EventType::StartElement, other)),
        }
    }

    /// Read an end tag that must match the supplied name.
    ///
    /// Advances the read cursor and is therefore not a idempotent operation.
    pub fn read_end_element(&mut self, name: Name<'_>) -> Result<(), DeError> {
        match self.read_event()? {
            Event::EndElement(end_element) => name
                .matches(end_element.borrow())
                .map_err(|err| DeError::from_inner(InnerError::InvalidName(Tag::End, err))),
            other => Err(DeError::invalid_event(EventType::StartElement, other)),
        }
    }
}

impl<R: Read> Deserializer<R> {
    /// Peek if the next XML token is a start element matching a supplied name
    pub fn peek_start_element_matches(&mut self, name: Name<'_>) -> bool {
        if let Ok(Some(BorrowedEvent::StartElement(start_element_name))) = self.peek_event() {
            name.matches(start_element_name.name.borrow()).is_ok()
        } else {
            false
        }
    }

    pub(crate) fn peek_event(&mut self) -> Result<Option<BorrowedEvent<'_>>, DeError> {
        // BUG: ?
        #[allow(unused_assignments)]
        let mut found_event = false;

        match self.event_stream().peek() {
            Some(peeked_xml_event) => match peeked_xml_event {
                Ok(xml_event) => found_event = BorrowedEvent::is_ximple_event(xml_event),
                Err(err) => return Err(DeError::from_reader(err.clone())),
            },
            None => return Ok(None),
        };

        match found_event {
            true => {
                // WORKAROUND: trying to get around borrow checker returning
                // directly in Ok(xml_event) match arm resulted in compiler
                // errors.
                let event = self.event_stream().peek().unwrap().as_ref().unwrap();
                Ok(BorrowedEvent::from_xml_event(event))
            }
            false => {
                self.event_stream().next();
                self.peek_event()
            }
        }
    }
}
