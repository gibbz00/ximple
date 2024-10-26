use ::xml::reader::XmlEvent;

use crate::*;

// TODO: Deserializer::next_event() -> Result<XimpleEvent> method
// returns error if is closed,
// whitespace unreachable (trim white space enabled)
// pull twice on: comments and process instruction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    StartElement(StartElement),
    EndElement(OwnedName),
    Characters(String),
    CData(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartElement {
    pub name: OwnedName,
    pub attributes: Vec<OwnedAttribute>,
    pub namespace: Namespace,
}

impl Event {
    pub(crate) fn from_xml_event(xml_event: XmlEvent) -> Option<Self> {
        let event = match xml_event {
            XmlEvent::StartElement { name, attributes, namespace } => Self::StartElement(StartElement { name, attributes, namespace }),
            XmlEvent::EndElement { name } => Self::EndElement(name),
            XmlEvent::CData(string) => Self::CData(string),
            XmlEvent::Characters(string) => Self::Characters(string),
            XmlEvent::StartDocument { .. } | XmlEvent::ProcessingInstruction { .. } | XmlEvent::Comment(_) | XmlEvent::EndDocument => {
                return None
            }
            XmlEvent::Whitespace(_) => unreachable!("reader should trim whitespaces"),
        };

        Some(event)
    }
}
