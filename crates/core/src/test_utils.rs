use crate::*;

pub fn assert_serialize_str<T: ToXml>(expected_str: &str, t: &T) {
    let actual = crate::to_string(t).unwrap();
    pretty_assertions::assert_eq!(expected_str, actual);
}

pub fn assert_deserialize_str<T: FromXml + std::fmt::Debug + PartialEq>(expected: &T, str: &str) {
    let actual = crate::from_str(str).unwrap();
    assert_eq!(expected, &actual);
}
