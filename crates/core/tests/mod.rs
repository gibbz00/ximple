#![allow(missing_docs)]

mod derives;
pub(crate) use derives::*;

#[test]
fn error_ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/**/*.rs");
}

mod pretty {
    use crate::*;

    #[derive(ximple::ToXml)]
    struct Foo {
        a: (),
        b: bool,
        c: &'static str,
    }

    #[test]
    fn to_string_pretty() {
        let expected_str = indoc::indoc! {"
            <a />
            <b>true</b>
            <c>test</c>"
        };

        let foo = Foo { a: (), b: true, c: "test" };

        assert_serialize_str_pretty(expected_str, &foo);
    }
}
