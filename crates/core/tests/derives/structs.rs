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

    #[derive(Debug, PartialEq, ximple::ToXml, ximple::FromXml)]
    struct FirstOptionStruct {
        first: Option<bool>,
        second: bool,
    }

    assert_bijective_xml!(
        first_field_option_some,
        "<first>true</first><second>false</second>",
        FirstOptionStruct { first: Some(true), second: false }
    );

    assert_bijective_xml!(
        first_field_option_none,
        "<second>true</second>",
        FirstOptionStruct { first: None, second: true }
    );

    #[derive(Debug, PartialEq, ximple::ToXml, ximple::FromXml)]
    struct GenericNamedStruct<T> {
        foo: T,
    }

    assert_bijective_xml!(generic, "<foo>true</foo>", GenericNamedStruct { foo: true });
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

    #[derive(Debug, PartialEq, ximple::ToXml, ximple::FromXml)]
    struct GenericContainer<T> {
        a: T,
    }

    #[derive(Debug, PartialEq, ximple::ToXml, ximple::FromXml)]
    struct GenericUnnamedStruct<T>(GenericContainer<T>);

    assert_bijective_xml!(generic, "<a>true</a>", GenericUnnamedStruct(GenericContainer { a: true }));
}
