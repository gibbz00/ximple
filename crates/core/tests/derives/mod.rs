mod enums;
mod structs;

pub use test_utils::{assert_serialize_str, assert_serialize_str_pretty};
mod test_utils {
    pub fn assert_serialize_str<T: ximple::ToXml>(expected_str: &str, t: &T) {
        let actual = ximple::to_string(t).unwrap();
        pretty_assertions::assert_eq!(expected_str, actual);
    }

    pub fn assert_serialize_str_pretty<T: ximple::ToXml>(expected_str: &str, t: &T) {
        let actual = ximple::to_string_pretty(t).unwrap();
        pretty_assertions::assert_eq!(expected_str, actual);
    }
}
