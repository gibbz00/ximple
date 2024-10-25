//! ximple - Simple XML serialization

/// Derive macro re-exports
pub use ximple_macros::{FromXml, FromXmlAttr, ToXml, ToXmlAttr};

/// `xml` re-exports
pub mod xml {
    pub use ::xml::common::XmlVersion;
}
pub(crate) use xml::*;

mod api;
pub use api::*;

pub mod ser;
pub(crate) use ser::{Error as SerError, Serializer};

mod to_xml;
pub use to_xml::ToXml;

pub mod provided;
pub(crate) use provided::*;

#[cfg(test)]
pub(crate) mod test_utils;
#[cfg(test)]
pub(crate) use test_utils::MockXml;
