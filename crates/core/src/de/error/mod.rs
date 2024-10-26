mod public;
pub use public::Error;

mod inner;
pub(crate) use inner::InnerError;

mod event_type;
pub use event_type::EventType;
pub(crate) use event_type::PrivateEventType;

mod tag;
pub(crate) use tag::Tag;
