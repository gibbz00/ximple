/// Element name
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Name<'a> {
    prefix: Option<&'a str>,
    name: &'a str,
}

impl<'a> Name<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { prefix: None, name }
    }

    pub fn with_prefix(name: &'a str, prefix: &'a str) -> Self {
        Self { prefix: Some(prefix), name }
    }
}

impl Name<'_> {
    pub(crate) fn matches(&self, xml_name: ::xml::name::Name<'_>) -> Result<(), InvalidNameError> {
        if self.name != xml_name.local_name {
            return Err(InvalidNameError::Name {
                expected: self.name.to_string(),
                found: xml_name.local_name.to_string(),
            });
        }

        if self.prefix != xml_name.prefix {
            return Err(InvalidNameError::Prefix {
                expected: self.prefix.map(ToOwned::to_owned),
                found: xml_name.prefix.map(ToOwned::to_owned),
            });
        }

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) enum InvalidNameError {
    Name { expected: String, found: String },
    Prefix { expected: Option<String>, found: Option<String> },
}

impl std::fmt::Display for InvalidNameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (object, expected, found) = match self {
            InvalidNameError::Name { expected, found } => ("names", Some(expected), Some(found)),
            InvalidNameError::Prefix { expected, found } => ("prefixes", expected.as_ref(), found.as_ref()),
        };

        write!(f, "{} don't match,", object)?;

        let mut write_helper = |verb: &str, value: Option<&String>| match value {
            Some(value) => write!(f, " {} '{}'", verb, value),
            None => write!(f, " none {}", verb),
        };

        write_helper("expected", expected)?;
        write_helper("found", found)
    }
}

impl std::error::Error for InvalidNameError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_xml_name() -> ::xml::name::Name<'static> {
        ::xml::name::Name { local_name: "bar", namespace: None, prefix: Some("foo") }
    }

    fn mock_name() -> Name<'static> {
        Name { prefix: Some("foo"), name: "bar" }
    }

    #[test]
    fn matches_xml_name() {
        let name = mock_name();
        let xml_name = mock_xml_name();
        assert!(name.matches(xml_name).is_ok());

        let name = Name { prefix: Some("other"), ..mock_name() };
        assert!(matches!(name.matches(xml_name).unwrap_err(), InvalidNameError::Prefix { .. }));

        let name = Name { name: "other", ..mock_name() };
        assert!(matches!(name.matches(xml_name).unwrap_err(), InvalidNameError::Name { .. }));

        // XML namespace doesn't matter
        let name = mock_name();
        let xml_name = ::xml::name::Name { namespace: Some("other"), ..mock_xml_name() };
        assert!(name.matches(xml_name).is_ok());
    }
}
