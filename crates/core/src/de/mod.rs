mod error;
pub use error::Error;

pub mod event;
pub(crate) use event::{Event, StartElement};

mod deserializer {
    use std::io::Read;

    pub struct Deserializer<R: Read> {
        event_reader: ::xml::EventReader<R>,
    }

    impl<R: Read> Deserializer<R> {
        pub(crate) fn new(reader: R) -> Self {
            let event_reader = ::xml::ParserConfig::new().trim_whitespace(true).create_reader(reader);
            Self { event_reader }
        }

        /// Used to deserialize special elements provided by this crate.
        /// Ex. [`Document`](crate::provided::Document)
        pub(crate) fn event_reader(&mut self) -> &mut ::xml::EventReader<R> {
            &mut self.event_reader
        }
    }
}
pub use deserializer::Deserializer;
