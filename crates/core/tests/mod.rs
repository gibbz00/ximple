#![allow(missing_docs)]

#[test]
fn error_ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}

pub use test_utils::assert_serialize_str;
mod test_utils {
    pub fn assert_serialize_str<T: ximple::ToXml>(expected_str: &str, t: &T) {
        let actual = ximple::to_string(t).unwrap();
        pretty_assertions::assert_eq!(expected_str, actual);
    }
}

mod structs {
    use ximple::ToXml;

    mod unit {
        use super::*;
        use crate::assert_serialize_str;

        #[derive(ToXml)]
        struct UnitStruct;

        #[test]
        fn unit_noop() {
            assert_serialize_str("", &UnitStruct);
        }
    }
}
