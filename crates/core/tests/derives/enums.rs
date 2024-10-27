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

    assert_bijective_xml!("<A><a/><b>false</b></A>", NamedEnum::A { a: (), b: false });
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
        "<A><B>true</B><C>false</C></A>",
        UnnamedEnum::A(InnerUnnamedEnum::B(true), InnerUnnamedEnum::C(false))
    );
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
}
