mod unit {
    use crate::*;

    #[derive(Debug, PartialEq, ximple::ToXml, ximple::FromXml)]
    enum UnitEnum {
        A,
        B,
    }

    assert_bijective_xml!(variant_a, "<A/>", UnitEnum::A);
    assert_bijective_xml!(variant_b, "<B/>", UnitEnum::B);
}

mod named {
    use crate::*;

    #[derive(Debug, PartialEq, ximple::ToXml, ximple::FromXml)]
    enum NamedEnum {
        A { a: (), b: bool },
    }

    assert_bijective_xml!(base, "<A><a/><b>false</b></A>", NamedEnum::A { a: (), b: false });

    #[derive(Debug, PartialEq, ximple::ToXml, ximple::FromXml)]
    enum GenericNamedEnum<T> {
        A { a: T },
    }

    assert_bijective_xml!(generic, "<A><a>false</a></A>", GenericNamedEnum::A { a: false });
}

mod unnamed {
    use crate::*;

    #[derive(Debug, PartialEq, ximple::ToXml, ximple::FromXml)]
    enum UnnamedEnum {
        A(InnerUnnamedEnum, InnerUnnamedEnum),
    }

    #[derive(Debug, PartialEq, ximple::ToXml, ximple::FromXml)]
    enum InnerUnnamedEnum {
        B(bool),
        C(bool),
    }

    assert_bijective_xml!(
        base,
        "<A><B>true</B><C>false</C></A>",
        UnnamedEnum::A(InnerUnnamedEnum::B(true), InnerUnnamedEnum::C(false))
    );

    #[derive(Debug, PartialEq, ximple::ToXml, ximple::FromXml)]
    enum GenericUnnamedEnum<T> {
        A(T),
    }

    assert_bijective_xml!(generic, "<A>true</A>", GenericUnnamedEnum::A(true));
}

mod mixed {
    use crate::*;

    #[derive(Debug, PartialEq, ximple::ToXml, ximple::FromXml)]
    enum Mixed {
        A,
        B { b: bool },
        C(Box<Mixed>, Box<Mixed>),
    }

    assert_bijective_xml!(variant_a, "<A/>", Mixed::A);
    assert_bijective_xml!(variant_b, "<B><b>true</b></B>", Mixed::B { b: true });
    assert_bijective_xml!(
        variant_c,
        "<C><A/><B><b>false</b></B></C>",
        Mixed::C(Box::new(Mixed::A), Box::new(Mixed::B { b: false }))
    );

    #[derive(Debug, PartialEq, ximple::ToXml, ximple::FromXml)]
    enum GenericMixed<T> {
        A,
        B { b: T },
        C(Box<GenericMixed<T>>, Box<GenericMixed<T>>),
    }

    assert_bijective_xml!(generic_variant_a, "<A/>", GenericMixed::<()>::A);
    assert_bijective_xml!(generic_variant_b, "<B><b>true</b></B>", GenericMixed::B { b: true });
    assert_bijective_xml!(
        generic_variant_c,
        "<C><A/><B><b>false</b></B></C>",
        GenericMixed::C(Box::new(GenericMixed::A), Box::new(GenericMixed::B { b: false }))
    );
}
