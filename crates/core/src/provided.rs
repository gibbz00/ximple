mod document {
    use crate::*;

    /// XML document declaration
    ///
    /// ```xml
    /// <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
    /// ```
    ///
    /// Encoding hard-coded to "UTF-8" due to it being the only supported
    /// encoding in the underlying writer.
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

    #[cfg(test)]
    mod mocks {
        use super::*;

        impl damock::Mock for Document {
            fn mock() -> Self {
                Self { standalone: true, version: XmlVersion::Version11 }
            }
        }

        impl MockXml for Document {
            fn mock_xml() -> String {
                r#"<?xml version="1.1" encoding="UTF-8" standalone="yes"?>"#.to_string()
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn serialization() {
            assert_serialize_mock::<Document>();
        }
    }
}
pub use document::Document;
