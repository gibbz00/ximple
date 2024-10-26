use crate::*;

/// Used in [`Error::invalid_event`]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventType {
    StartElement,
    EndElement,
    Characters,
    CData,
}

impl EventType {
    pub(crate) fn as_private(self) -> PrivateEventType {
        match self {
            EventType::StartElement => PrivateEventType::StartElement,
            EventType::EndElement => PrivateEventType::EndElement,
            EventType::Characters => PrivateEventType::Characters,
            EventType::CData => PrivateEventType::CData,
        }
    }
}

/// Superset of the public [`EventType`] used to include internal events
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum PrivateEventType {
    StartDocument,
    StartElement,
    EndElement,
    Characters,
    CData,
}

impl PrivateEventType {
    pub(crate) fn from_event(event: &Event) -> Self {
        match event {
            Event::StartElement(_) => PrivateEventType::StartElement,
            Event::EndElement(_) => PrivateEventType::EndElement,
            Event::Characters(_) => PrivateEventType::Characters,
            Event::CData(_) => PrivateEventType::CData,
        }
    }
}

impl std::fmt::Display for PrivateEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PrivateEventType::StartDocument => "start document",
            PrivateEventType::StartElement => "start element",
            PrivateEventType::EndElement => "end element",
            PrivateEventType::Characters => "characters",
            PrivateEventType::CData => "CDATA",
        })
    }
}
