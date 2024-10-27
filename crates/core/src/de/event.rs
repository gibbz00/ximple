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

pub(crate) use borrowed::BorrowedEvent;
mod borrowed {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum BorrowedEvent<'a> {
        StartElement(BorrowedStartElement<'a>),
        EndElement(&'a OwnedName),
        Characters(&'a String),
        CData(&'a String),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct BorrowedStartElement<'a> {
        pub name: &'a OwnedName,
        pub attributes: &'a Vec<OwnedAttribute>,
        pub namespace: &'a Namespace,
    }

    impl<'a> BorrowedEvent<'a> {
        pub(crate) fn is_ximple_event(xml_event: &'a XmlEvent) -> bool {
            match xml_event {
                XmlEvent::StartElement { .. } | XmlEvent::EndElement { .. } | XmlEvent::CData(_) | XmlEvent::Characters(_) => true,
                XmlEvent::StartDocument { .. } | XmlEvent::ProcessingInstruction { .. } | XmlEvent::Comment(_) | XmlEvent::EndDocument => {
                    false
                }
                XmlEvent::Whitespace(_) => unreachable!("reader should trim whitespaces"),
            }
        }

        pub(crate) fn from_xml_event(xml_event: &'a XmlEvent) -> Option<Self> {
            let event = match xml_event {
                XmlEvent::StartElement { name, attributes, namespace } => {
                    Self::StartElement(BorrowedStartElement { name, attributes, namespace })
                }
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
}
