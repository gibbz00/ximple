mod unit {
    use crate::*;

    #[derive(Debug, PartialEq, ximple::ToXml, ximple::FromXml)]
    enum UnitEnum {
        A,
        B,
    }

    assert_bijective_xml!("<A/>", UnitEnum::A);

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
        B { b: bool },
        C(Box<Mixed>, Box<Mixed>),
    }

    impl ::ximple::FromXml for Mixed {
        fn deserialize(deserializer: &mut ::ximple::de::Deserializer<impl std::io::Read>) -> Result<Self, ::ximple::de::Error> {
            let variant_name = ::ximple::types::Name::new("A");
            if deserializer.peek_element(variant_name) {
                deserializer.read_element::<()>(variant_name)?;
                return Ok(Mixed::A);
            }

            let variant_name = ::ximple::types::Name::new("B");
            if deserializer.peek_element(variant_name) {
                deserializer.read_start_element(variant_name);

                let value = Mixed::B { b: deserializer.read_element(::ximple::types::Name::new("b"))? };

                deserializer.read_end_element(variant_name);

                return Ok(value);
            }

            if deserializer.peek_element(::ximple::types::Name::new("C")) {
                return Ok(Mixed::C(
                    ::ximple::FromXml::deserialize(deserializer)?,
                    ::ximple::FromXml::deserialize(deserializer)?,
                ));
            }

            Err(::ximple::de::Error::element_not_found(["A", "B", "C"].map(ToString::to_string)))
        }
    }

    #[test]
    fn element_and_each_field() {
        let mixed_enum = Mixed::A;
        assert_serialize_str("<A/>", &mixed_enum);

        let mixed_enum = Mixed::B { b: true };
        assert_serialize_str("<B><b>true</b></B>", &mixed_enum);

        let mixed_enum = Mixed::C(Box::new(Mixed::A), Box::new(Mixed::B { b: false }));
        assert_serialize_str("<C><A/><B><b>false</b></B></C>", &mixed_enum);
    }
}
