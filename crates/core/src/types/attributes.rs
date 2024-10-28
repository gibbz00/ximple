use std::{borrow::Cow, collections::HashMap};

/// Collection of unique element attributes
#[derive(Debug, PartialEq, Default)]
pub struct Attributes<'a>(HashMap<&'static str, Cow<'a, str>>);

impl Attributes<'_> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &str)> {
        self.0.iter().map(|(name, value)| (*name, value.as_ref()))
    }
}

impl<'a> Attributes<'a> {
    /// Insert an attribute to the element collection
    ///
    /// # Errors
    /// - If a attribute of the same name already exists within the collection.
    pub fn add(&mut self, name: &'static str, value: impl Into<Cow<'a, str>>) -> Result<(), AttributeDuplicate> {
        let value = value.into();

        if let Some(previous_value) = self.0.get(name) {
            return Err(AttributeDuplicate {
                name,
                previous_value: previous_value.to_string(),
                new_value: value.to_string(),
            });
        };

        self.0.insert(name, value);

        Ok(())
    }
}

pub mod error {
    #[derive(Debug, thiserror::Error)]
    pub enum AttributeError {
        #[error(transparent)]
        InsertDuplicate(#[from] AttributeDuplicate),
    }

    #[derive(Debug, PartialEq, thiserror::Error)]
    #[error(
        "attempted to add a duplicate attribute, name: '{}', previous value: '{}', new value: '{}'",
        .name, .previous_value, .new_value
    )]
    pub struct AttributeDuplicate {
        pub name: &'static str,
        pub previous_value: String,
        pub new_value: String,
    }
}
pub(crate) use error::{AttributeDuplicate, AttributeError};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let name = "foo";
        let value = "bar";

        let mut attributes = Attributes::new();
        assert!(attributes.add(name, value).is_ok());

        let new_value = "baz";
        let expected_error = AttributeDuplicate {
            name,
            previous_value: value.to_string(),
            new_value: new_value.to_string(),
        };

        let actual_error = attributes.add(name, new_value).unwrap_err();
        assert_eq!(expected_error, actual_error);
    }
}
