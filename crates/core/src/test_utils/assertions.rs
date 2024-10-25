use crate::*;

pub fn assert_serialize_mock<T: ToXml + damock::Mock + MockXml>() {
    assert_serialize_str(&T::mock_xml(), &T::mock());
}

pub fn assert_serialize_str<T: ToXml>(expected_str: &str, t: &T) {
    let actual = crate::to_string(t).unwrap();
    pretty_assertions::assert_eq!(expected_str, actual);
}
