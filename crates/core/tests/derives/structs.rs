mod unit {
    use crate::*;

    #[derive(Debug, PartialEq, ximple::ToXml, ximple::FromXml)]
    struct UnitStruct;

    assert_bijective_xml!(noop, "", UnitStruct);
}

mod named {
    use crate::*;

    #[derive(Debug, PartialEq, ximple::ToXml, ximple::FromXml)]
    struct NamedStruct {
        foo: bool,
        bar: (),
    }

    assert_bijective_xml!(fields_create_elements, "<foo>true</foo><bar/>", NamedStruct { foo: true, bar: () });
}

mod unnamed {
    use crate::*;

    #[derive(Debug, PartialEq, ximple::ToXml, ximple::FromXml)]
    struct Container {
        a: bool,
    }

    #[derive(Debug, PartialEq, ximple::ToXml, ximple::FromXml)]
    struct UnnamedStruct(Container, Container);

    assert_bijective_xml!(
        fields_create_elements,
        "<a>true</a><a>false</a>",
        UnnamedStruct(Container { a: true }, Container { a: false })
    );
}
