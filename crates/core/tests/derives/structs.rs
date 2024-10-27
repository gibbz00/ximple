mod unit {
    use crate::*;

    #[derive(Debug, PartialEq, ximple::ToXml, ximple::FromXml)]
    struct UnitStruct;

    assert_bijective_xml!(noop, "", UnitStruct);
}

mod named {
    use ximple::ToXml;

    use crate::*;

    #[derive(ToXml)]
    struct NamedStruct {
        foo: &'static str,
        bar: (),
    }

    #[test]
    fn fields_create_elements() {
        assert_serialize_str("<foo>test</foo><bar/>", &NamedStruct { foo: "test", bar: () });
    }
}

mod unnamed {
    use ximple::ToXml;

    use crate::*;

    #[derive(ToXml)]
    struct Container {
        a: &'static str,
    }

    #[derive(ToXml)]
    struct UnnamedStruct(Container, Container);

    #[test]
    fn fields_create_elements() {
        let unnamed_struct = UnnamedStruct(Container { a: "foo" }, Container { a: "bar" });
        assert_serialize_str("<a>foo</a><a>bar</a>", &unnamed_struct);
    }
}
