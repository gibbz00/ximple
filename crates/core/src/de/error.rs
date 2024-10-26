use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("XML reader error")]
    Reader(#[from] ::xml::reader::Error),
    #[error(
        "deserialized unexpected event: expected {}, found: {}",
        display_event_for_error(.expected),
        display_event_for_error(.found))
    ]
    UnexpectedEvent { expected: Event, found: Event },
    #[error("reached end of document")]
    End,
}

fn display_event_for_error(event: &Event) -> String {
    let (name, content) = match event {
        Event::StartElement(start_element) => ("start element", start_element.name.to_string()),
        Event::EndElement(owned_name) => ("end element", owned_name.to_string()),
        Event::Characters(string) => ("characters", string.clone()),
        Event::CData(string) => ("CDATA", string.clone()),
    };

    format!("{} containing '{}'", name, content)
}
