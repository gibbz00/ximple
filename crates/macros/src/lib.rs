//! Derive proc-macro definitions for the `ximple` crate.

use proc_macro::TokenStream;

mod attribute_utils;
pub(crate) use attribute_utils::{AttributeBuffer, AttributeParseError, AttributeParser, XimpleAttribute};

mod derive_from_xml;
mod derive_to_xml;

/// Derives `ToXml`
#[proc_macro_derive(ToXml, attributes(ximple))]
pub fn derive_to_xml(token_stream: TokenStream) -> TokenStream {
    derive_to_xml::derive_to_xml_impl(token_stream)
}

/// Derives `FromXml`
#[proc_macro_derive(FromXml, attributes(ximple))]
pub fn derive_from_xml(token_stream: TokenStream) -> TokenStream {
    derive_from_xml::derive_from_xml_impl(token_stream)
}

/// Derives `ToXmlAttr`
#[proc_macro_derive(ToXmlAttr, attributes(ximple))]
pub fn derive_to_xml_attr(_token_stream: TokenStream) -> TokenStream {
    todo!()
}

/// Derives `FromXmlAttr`
#[proc_macro_derive(FromXmlAttr, attributes(ximple))]
pub fn derive_from_xml_attr(_token_stream: TokenStream) -> TokenStream {
    todo!()
}
