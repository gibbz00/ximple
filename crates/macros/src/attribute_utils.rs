use std::collections::HashMap;

use syn::{meta::ParseNestedMeta, spanned::Spanned, Attribute};

use crate::*;

type AttributeBuffer<A> = HashMap<&'static str, A>;

pub trait CrateAttribute: Sized {
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

    /// Parse container or field attributes
    ///
    /// Sanitizes and reports errors on non-metalist, duplicate,
    /// incompatible, and unknown attributes.
    pub fn parse<A: CrateAttribute>(
        attributes: Vec<Attribute>,
        compatibility_context: A::CompatibilityContext,
    ) -> syn::Result<A::AttributeSet> {
        let mut attribute_buffer = AttributeBuffer::default();

        for crate_attr in attributes
            .into_iter()
            .filter(|attribute| attribute.meta.path().is_ident(Self::CRATE_PATH))
        {
            match crate_attr.meta {
                syn::Meta::List(meta_list) => {
                    meta_list.parse_nested_meta(|nested_meta| {
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
