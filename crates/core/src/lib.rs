//! ximple - Simple XML serialization

/// Derive macro re-exports
pub use ximple_macros::{FromXml, FromXmlAttr, ToXml, ToXmlAttr};

/// `xml` re-exports
pub mod xml {
    pub use ::xml::{attribute::OwnedAttribute, common::XmlVersion, name::OwnedName, namespace::Namespace};
}
pub(crate) use xml::*;

mod api;
pub use api::*;

pub mod ser;
pub(crate) use ser::{Error as SerError, *};

pub mod de;
pub(crate) use de::{Error as DeError, *};

mod traits;
pub use traits::{FromXml, ToXml, ToXmlAttr};

mod impls;

pub mod provided;

pub mod types;
pub(crate) use types::*;

#[cfg(test)]
pub(crate) mod test_utils;
#[cfg(test)]
pub(crate) use test_utils::{assert_bijective_xml, assert_deserialize_str, assert_serialize_str, Container};
