mod error;
pub(crate) use error::*;
pub use error::{Error, EventType};

pub mod event;
pub(crate) use event::Event;

mod deserializer;
pub use deserializer::Deserializer;

mod stream;
pub(crate) use stream::EventStream;
