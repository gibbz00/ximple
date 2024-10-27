mod enums;
mod structs;

pub use test_utils::{assert_deserialize_str, assert_serialize_str, assert_serialize_str_pretty};
mod test_utils {
    pub fn assert_serialize_str<T: ximple::ToXml>(expected_str: &str, t: &T) {
        let actual = ximple::to_string(t).unwrap();
        pretty_assertions::assert_eq!(expected_str, actual);
    }

    pub fn assert_serialize_str_pretty<T: ximple::ToXml>(expected_str: &str, t: &T) {
        let actual = ximple::to_string_pretty(t).unwrap();
        pretty_assertions::assert_eq!(expected_str, actual);
    }

    pub fn assert_deserialize_str<T: ximple::FromXml + std::fmt::Debug + PartialEq>(expected: &T, str: &str) {
        let actual = ximple::from_str(str).unwrap();
        assert_eq!(expected, &actual);
    }

    #[macro_export]
    macro_rules! assert_bijective_xml {
        ($ident:ident, $xml_str:expr, $value:expr) => {
            mod $ident {
                use super::*;

                assert_bijective_xml!($xml_str, $value);
            }
        };
        ($xml_str:expr, $value:expr) => {
            #[test]
            fn serialization() {
                $crate::assert_serialize_str($xml_str, &$value);
            }
            #[test]
            fn deserialization() {
                $crate::assert_deserialize_str(&$value, $xml_str);
            }
        };
    }
}
