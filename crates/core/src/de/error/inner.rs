use std::borrow::Cow;

use crate::*;

#[derive(thiserror::Error)]
pub enum InnerError {
    #[error("XML reader error")]
    Reader(::xml::reader::Error),
    #[error("invalid event: expected {}, found: {}", .0, Self::display_found(.1))]
    InvalidEvent(PrivateEventType, Event),
    #[error("reached end of document")]
    End,
    #[error("unable to read {} tag, invalid name", .0)]
    InvalidName(Tag, #[source] InvalidNameError),
    #[error("invalid value, expected '{}', found: '{}'", .0, .1)]
    InvalidValue(Cow<'static, str>, String),
    #[error("no {} element found", Self::display_elements(.0.as_slice()))]
    ElementNotFound(Vec<String>),
}

impl std::fmt::Debug for InnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
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

    fn display_elements(elements: &[String]) -> String {
        match elements.len() {
            0 => "".to_string(),
            1 => format!("<{}>", elements[0]),
            _ => {
                let mut element_iterator = elements.iter().peekable();
                let mut elements_string = String::new();
                loop {
                    let element = element_iterator.next().expect("missing element");

                    match element_iterator.peek().is_some() {
                        true => {
                            elements_string.push('<');
                            elements_string.push_str(element);
                            elements_string.push_str(">, ");
                        }
                        false => {
                            // remove Oxford comma
                            elements_string.pop();
                            elements_string.pop();

                            elements_string.push_str(" or <");
                            elements_string.push_str(element);
                            elements_string.push('>');

                            break;
                        }
                    }
                }

                elements_string
            }
        }
    }
}
