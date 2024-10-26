mod unit {
    use crate::*;

    #[derive(ximple::ToXml)]
    enum UnitEnum {
        A,
        B,
    }

    #[test]
    fn element_for_variant() {
        assert_serialize_str("<A/>", &UnitEnum::A);
        assert_serialize_str("<B/>", &UnitEnum::B);
    }
}

mod named {
    use crate::*;

    #[derive(ximple::ToXml)]
    enum NamedEnum {
        A { a: (), b: &'static str },
    }

    #[test]
    fn element_for_variant_and_fields() {
        let named_enum = NamedEnum::A { a: (), b: "test" };
        assert_serialize_str("<A><a/><b>test</b></A>", &named_enum);
    }
}

mod unnamed {
    use crate::*;

    #[derive(ximple::ToXml)]
    enum UnnamedEnum {
        A(InnerUnnamedEnum, InnerUnnamedEnum),
    }

    #[derive(ximple::ToXml)]
    enum InnerUnnamedEnum {
        B(&'static str),
        C(&'static str),
    }

    #[test]
    fn element_for_variant_but_not_fields() {
        let unnamed_enum = UnnamedEnum::A(InnerUnnamedEnum::B("bb"), InnerUnnamedEnum::C("cc"));
        assert_serialize_str("<A><B>bb</B><C>cc</C></A>", &unnamed_enum);
    }
}

mod mixed {
    use crate::*;

    #[derive(ximple::ToXml)]
    enum Mixed {
        A,
        B { b: &'static str },
        C(&'static str),
    }

    #[test]
    fn element_and_each_field() {
        let mixed_enum = Mixed::A;
        assert_serialize_str("<A/>", &mixed_enum);

        let mixed_enum = Mixed::B { b: "test" };
        assert_serialize_str("<B><b>test</b></B>", &mixed_enum);

        let mixed_enum = Mixed::C("test");
        assert_serialize_str("<C>test</C>", &mixed_enum);
    }
}
