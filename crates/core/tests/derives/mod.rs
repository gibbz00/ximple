pub use test_utils::assert_serialize_str;
mod test_utils {
    pub fn assert_serialize_str<T: ximple::ToXml>(expected_str: &str, t: &T) {
        let actual = ximple::to_string(t).unwrap();
        pretty_assertions::assert_eq!(expected_str, actual);
    }
}

mod structs;

mod enums;
