use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("XML writer error")]
    EventWriter(#[from] ::xml::writer::Error),
    #[error("attribute error")]
    Attribute(#[from] AttributeError),
}
