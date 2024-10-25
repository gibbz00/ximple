#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("XML writer error")]
    EventWriter(#[from] ::xml::writer::Error),
}
