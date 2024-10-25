mod mock_xml;
pub use mock_xml::MockXml;

mod assertions;
pub use assertions::{assert_serialize_mock, assert_serialize_str};
