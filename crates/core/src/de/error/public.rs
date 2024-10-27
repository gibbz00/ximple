use std::borrow::Cow;

use crate::*;

pub struct Error {
    inner: InnerError,
}

impl Error {
    pub fn invalid_event(expected: EventType, found: Event) -> Self {
        Self { inner: InnerError::InvalidEvent(expected.as_private(), found) }
    }

    pub fn invalid_value(expected: impl Into<Cow<'static, str>>, found: impl Into<String>) -> Self {
        Self { inner: InnerError::InvalidValue(expected.into(), found.into()) }
    }

    pub fn element_not_found(expected: impl Into<Vec<String>>) -> Self {
        Self { inner: InnerError::ElementNotFound(expected.into()) }
    }

    pub(crate) fn end() -> Self {
        Self { inner: InnerError::End }
    }

    pub(crate) fn from_reader(reader_error: ::xml::reader::Error) -> Self {
        Self { inner: InnerError::Reader(reader_error) }
    }

    pub(crate) fn from_inner(inner: InnerError) -> Self {
        Self { inner }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.inner, f)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.inner, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}
