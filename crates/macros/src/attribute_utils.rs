use std::collections::HashMap;

use syn::{meta::ParseNestedMeta, spanned::Spanned, Attribute, Ident};

pub enum DeriveName {
    ToXml,
    FromXml,
}

impl DeriveName {
    fn as_str(&self) -> &'static str {
        match self {
            DeriveName::ToXml => "ToXml",
            DeriveName::FromXml => "FromXml",
        }
    }
}

pub struct ContainerCompatibilityContext;

pub struct FieldCompatibilityContext<A> {
    container_attribute_set: A,
    other_derive_is_present: bool,
}

pub type AttributeBuffer<A> = HashMap<&'static str, A>;

pub trait XimpleAttribute: Sized {
    const DERIVE_NAME: DeriveName;

    /// Set of compatible attributes, usually represented with an enum
    ///
    /// Used in [`Self::into_compatible_set`]
    type AttributeSet;
    /// Context needed to decide on which attributes are compatible
    ///
    /// Usually `None` for container attributes, and the container
    /// attributes when parsing field attributes.
    ///
    /// Used in [`Self::into_compatible_set`]
    type CompatibilityContext;

    /// Unique attribute name, used as the key in [`AttributeParser::buffer`]
    fn name(&self) -> &'static str;

    /// Check if a given ident belongs to self
    fn is_name(ident: &Ident) -> bool;

    fn parse_attribute(nested_meta: ParseNestedMeta<'_>) -> Result<Self, AttributeParseError>;

    fn into_compatible_set(
        buffer: AttributeBuffer<Self>,
        context: Self::CompatibilityContext,
    ) -> Result<Self::AttributeSet, AttributeParseError>;
}

pub struct AttributeParser;

impl AttributeParser {
    const CRATE_PATH: &str = "ximple";
    const EXPECTED_ATTRIBUTE_SYNTAX: &str = "invalid attribute syntax, expected #[ximple(...)]";

    /// Parse container attributes
    ///
    /// Sanitizes and reports errors on non-metalist, duplicate,
    /// incompatible, and unknown attributes.
    ///
    /// O paratameter is supplied to tell the parser if it should skip
    /// invalidating attributes used by another ximple derive macro.
    pub fn parse_container_attributes<A: XimpleAttribute<CompatibilityContext = ContainerCompatibilityContext>, O: XimpleAttribute>(
        attributes: Vec<Attribute>,
    ) -> syn::Result<A::AttributeSet> {
        let other_derive_is_present = Self::derive_is_present(O::DERIVE_NAME, &attributes);

        Self::parse::<A, O>(attributes, ContainerCompatibilityContext, other_derive_is_present)
    }

    /// Parse field attributes
    ///
    /// Sanitizes and reports errors on non-metalist, duplicate,
    /// incompatible, and unknown attributes.
    pub fn parse_field_attributes<A: XimpleAttribute, O: XimpleAttribute>(
        attributes: Vec<Attribute>,
        compatibility_context: FieldCompatibilityContext<A::CompatibilityContext>,
    ) -> syn::Result<A::AttributeSet> {
        Self::parse::<A, O>(
            attributes,
            compatibility_context.container_attribute_set,
            compatibility_context.other_derive_is_present,
        )
    }

    fn parse<A: XimpleAttribute, O: XimpleAttribute>(
        attributes: Vec<Attribute>,
        compatibility_context: A::CompatibilityContext,
        other_derive_is_present: bool,
    ) -> syn::Result<A::AttributeSet> {
        let mut attribute_buffer = AttributeBuffer::default();

        for crate_attr in attributes
            .into_iter()
            .filter(|attribute| attribute.meta.path().is_ident(Self::CRATE_PATH))
        {
            match crate_attr.meta {
                syn::Meta::List(meta_list) => {
                    meta_list.parse_nested_meta(|nested_meta| {
                        let nested_meta_ident = nested_meta.path.require_ident()?;

                        if !A::is_name(nested_meta_ident) && other_derive_is_present && O::is_name(nested_meta_ident) {
                            return Ok(());
                        }

                        let crate_attribute_span = nested_meta.path.span();
                        let crate_attribute = A::parse_attribute(nested_meta)?;
                        match attribute_buffer.insert(crate_attribute.name(), crate_attribute).is_some() {
                            true => Err(AttributeParseError::duplicate(crate_attribute_span))?,
                            false => Ok(()),
                        }
                    })?;
                }
                syn::Meta::Path(path) => return Err(syn::Error::new(path.span(), Self::EXPECTED_ATTRIBUTE_SYNTAX)),
                syn::Meta::NameValue(meta_name_value) => {
                    return Err(syn::Error::new(meta_name_value.span(), Self::EXPECTED_ATTRIBUTE_SYNTAX))
                }
            }
        }

        A::into_compatible_set(attribute_buffer, compatibility_context).map_err(Into::into)
    }

    fn derive_is_present(derive_name: DeriveName, attributes: &[Attribute]) -> bool {
        let derive_name_str = derive_name.as_str();

        attributes
            .iter()
            .filter(|attribute| attribute.path().is_ident("derive"))
            .filter_map(|attribute| attribute.meta.require_list().ok())
            .any(|derive_meta_list| {
                let mut found_derive = false;
                derive_meta_list
                    .parse_nested_meta(|nested_meta| {
                        found_derive = nested_meta.path.is_ident(derive_name_str);
                        Ok(())
                    })
                    .expect("infallible nested meta parser");
                found_derive
            })
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn finds_derive() {
        let input: Vec<Attribute> = vec![parse_quote!(#[test = "path"]), parse_quote!(#[derive(Other, ToXml)])];

        let derive_present = AttributeParser::derive_is_present(DeriveName::ToXml, &input);
        assert!(derive_present);

        let derive_not_present = AttributeParser::derive_is_present(DeriveName::FromXml, &input);
        assert!(!derive_not_present);
    }
}

pub(crate) use error::AttributeParseError;
mod error {
    use proc_macro2::Span;

    pub struct AttributeParseError {
        span: Span,
        reason: AttributeParseErrorReason,
    }

    impl AttributeParseError {
        pub fn unknown(attribute_span: Span) -> Self {
            Self { span: attribute_span, reason: AttributeParseErrorReason::Unknown }
        }

        pub fn duplicate(duplicate_attribute_span: Span) -> Self {
            Self {
                span: duplicate_attribute_span,
                reason: AttributeParseErrorReason::Duplicate,
            }
        }
    }

    impl From<AttributeParseError> for syn::Error {
        fn from(error: AttributeParseError) -> Self {
            Self::new(error.span, error.reason.message())
        }
    }

    pub enum AttributeParseErrorReason {
        Unknown,
        Duplicate,
    }

    impl AttributeParseErrorReason {
        fn message(&self) -> &str {
            match self {
                AttributeParseErrorReason::Unknown => "unknown attribute",
                AttributeParseErrorReason::Duplicate => "duplicate attribute found",
            }
        }
    }
}
