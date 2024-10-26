use std::{io::Read, iter::Peekable};

use ::xml::reader::XmlEvent;

///  Wraps a raw XML event stream with the indent of keeping track of a peakable depth.
pub struct EventStream<R: Read> {
    depth: usize,
    inner: Peekable<::xml::reader::Events<R>>,
}

impl<R: Read> EventStream<R> {
    pub(crate) fn new(reader: R) -> Self {
        Self {
            depth: 0,
            inner: ::xml::ParserConfig::new()
                .trim_whitespace(true)
                .create_reader(reader)
                .into_iter()
                .peekable(),
        }
    }

    /// Retrieve the next event inner XML event iterator, adjusting the tracked depth if necessary
    pub(crate) fn next(&mut self) -> Option<Result<::xml::reader::XmlEvent, ::xml::reader::Error>> {
        let maybe_event = self.inner.next();

        if let Some(Ok(event)) = &maybe_event {
            match event {
                XmlEvent::StartElement { .. } => self.depth += 1,
                XmlEvent::EndElement { .. } => self.depth -= 1,
                _ => {}
            }
        }

        maybe_event
    }

    /// Return the tracked depth
    pub(crate) fn depth(&self) -> usize {
        self.depth
    }

    /// Peek the next element, does not affect the tracked depth
    pub(crate) fn peek(&mut self) -> Option<&Result<XmlEvent, xml::reader::Error>> {
        self.inner.peek()
    }

    /// Retrieve the resulting depth if [`Self::next`] is called.
    pub(crate) fn peek_depth(&mut self) -> usize {
        let maybe_event = self.inner.peek();

        let mut peeked_depth = self.depth;

        if let Some(Ok(event)) = maybe_event {
            match event {
                XmlEvent::StartElement { .. } => peeked_depth += 1,
                XmlEvent::EndElement { .. } => peeked_depth -= 1,
                _ => {}
            };
        }

        peeked_depth
    }
}
