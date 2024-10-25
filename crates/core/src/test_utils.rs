pub trait MockXml {
    fn mock_xml() -> String;
}

pub use assertions::assert_serialization;
mod assertions {
    use crate::*;

    pub fn assert_serialization<T: ToXml + damock::Mock + MockXml>() {
        let expected = T::mock_xml();
        let actual = crate::to_string(&T::mock()).unwrap();
        pretty_assertions::assert_eq!(expected, actual);
    }
}
