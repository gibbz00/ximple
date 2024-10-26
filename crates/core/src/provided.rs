mod document {
    use crate::*;

    /// XML document declaration
    ///
    /// ```xml
    /// <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
    /// ```
    ///
    /// Written encoding hard-coded to "UTF-8" due to it being the only
    /// supported encoding in the underlying writer.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Document {
        /// standalone="yes/no" attribute
        ///
        /// Attribute omitted in `ToXml::serialize` if set to false.
        pub standalone: bool,
        /// version="1.0/1.1" attribute
        pub version: XmlVersion,
    }

    impl ToXml for Document {
        fn serialize(&self, serializer: &mut Serializer<impl std::io::Write>) -> Result<(), SerError> {
            serializer
                .event_writer()
                .write(::xml::writer::XmlEvent::StartDocument {
                    version: self.version,
                    // Defaults to "UTF-8". Only supported writer encoding in `xml` crate.
                    encoding: None,
                    standalone: self.standalone.then_some(true),
                })
                .map_err(Into::into)
        }
    }

    impl FromXml for Document {
        fn deserialize(deserializer: &mut Deserializer<impl std::io::Read>) -> Result<Self, DeError> {
            match deserializer.event_reader().next()? {
                ::xml::reader::XmlEvent::StartDocument { version, standalone, .. } => {
                    Ok(Self { standalone: standalone.unwrap_or_default(), version })
                }
                xml_event => Err(match Event::from_xml_event(xml_event)? {
                    Some(event) => DeError::UnexpectedEvent {
                        // HACK: don't want to expose `StartDocument` in `Event` as it is provided by crate
                        expected: Event::StartElement(StartElement {
                            name: OwnedName { local_name: "xml".to_string(), namespace: None, prefix: None },
                            attributes: vec![],
                            namespace: Namespace::empty(),
                        }),
                        found: event,
                    },
                    None => DeError::End,
                }),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn mock_document() -> Document {
            Document { standalone: true, version: XmlVersion::Version11 }
        }

        fn mock_document_xml() -> String {
            r#"<?xml version="1.1" encoding="UTF-8" standalone="yes"?>"#.to_string()
        }

        #[test]
        fn serialization() {
            assert_serialize_str(&mock_document_xml(), &mock_document());
        }

        #[test]
        fn deserialization() {
            assert_deserialize_str(&mock_document(), &mock_document_xml());
        }
    }
}
pub use document::Document;
