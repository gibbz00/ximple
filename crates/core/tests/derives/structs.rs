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
