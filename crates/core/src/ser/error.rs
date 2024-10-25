#[derive(Debug)]
pub enum Error {
    EventWriter(::xml::writer::Error),
}
