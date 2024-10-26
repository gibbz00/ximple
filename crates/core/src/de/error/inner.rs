use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum InnerError {
    #[error("XML reader error")]
    Reader(::xml::reader::Error),
    #[error("invalid event: expected {}, found: {}", .0, Self::display_found(.1))]
    InvalidEvent(PrivateEventType, Event),
    #[error("reached end of document")]
    End,
    #[error("unable to read {} tag, invalid name", .0)]
    InvalidName(Tag, #[source] InvalidNameError),
}

impl InnerError {
    fn display_found(event: &Event) -> String {
        let name = PrivateEventType::from_event(event);
        let content = match event {
            Event::StartElement(start_element) => start_element.name.to_string(),
            Event::EndElement(owned_name) => owned_name.to_string(),
            Event::Characters(string) => string.clone(),
            Event::CData(string) => string.clone(),
        };

        format!("{} containing '{}'", name, content)
    }
}
