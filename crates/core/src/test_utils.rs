use crate::*;

pub fn assert_serialize_str<T: ToXml>(expected_str: &str, t: &T) {
    let actual = crate::to_string(t).unwrap();
    pretty_assertions::assert_eq!(expected_str, actual);
}

pub fn assert_deserialize_str<T: FromXml + std::fmt::Debug + PartialEq>(expected: &T, str: &str) {
    let actual = crate::from_str(str).unwrap();
    assert_eq!(expected, &actual);
}

pub mod container {
    use crate::*;

    #[derive(PartialEq, Debug)]
    pub struct Container<T>(pub T);

    pub const ELEMENT_NAME: &str = "a";

    pub fn contained_xml(str: &str) -> String {
        format!("<{}>{}</{}>", ELEMENT_NAME, str, ELEMENT_NAME)
    }

    impl<T: ToXml> ToXml for Container<T> {
        fn serialize(&self, serializer: &mut Serializer<impl std::io::Write>) -> Result<(), SerError> {
            serializer.write_element(ELEMENT_NAME, &self.0)
        }
    }

    impl<T: FromXml> FromXml for Container<T> {
        fn deserialize(deserializer: &mut Deserializer<impl std::io::Read>) -> Result<Self, DeError> {
            let inner = deserializer.read_element(Name::new(ELEMENT_NAME))?;
            Ok(Self(inner))
        }
    }
}
pub use container::Container;

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
            assert_serialize_str($xml_str, &$value);
        }
        #[test]
        fn deserialization() {
            let container = $crate::Container($value);
            let contained_xml = $crate::test_utils::container::contained_xml($xml_str);
            assert_deserialize_str(&container, &contained_xml);
        }
    };
}
pub(crate) use assert_bijective_xml;
