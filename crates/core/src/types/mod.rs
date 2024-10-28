mod name;
pub(crate) use name::InvalidNameError;
pub use name::Name;

mod attributes;
pub(crate) use attributes::AttributeError;
pub use attributes::{error, Attributes};
