use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DataEnum, FieldsNamed, FieldsUnnamed, Ident};

pub fn derive(enum_ident: Ident, data_enum: DataEnum) -> TokenStream2 {
    // TODO: note that this this must take future 'skip', 'rename' and 'flatten'
    // attributes into account too.
    let not_found_error = {
        let variant_names = data_enum.variants.iter().map(|variant| variant.ident.to_string());
        quote! { Err(::ximple::de::Error::element_not_found([#(#variant_names),*].map(ToString::to_string))); }
    };

    let match_arms = data_enum.variants.into_iter().map(|variant| match variant.fields {
        syn::Fields::Named(fields_named) => named(&enum_ident, &variant.ident, fields_named),
        syn::Fields::Unnamed(fields_unnamed) => unnamed(&enum_ident, &variant.ident, fields_unnamed),
        syn::Fields::Unit => unit(&enum_ident, &variant.ident),
    });

    quote! {
    impl ::ximple::FromXml for #enum_ident {
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

                #not_found_error
            }
        }
    }
}

fn unit(enum_ident: &Ident, variant_ident: &Ident) -> TokenStream2 {
    todo!()
}

fn unnamed(enum_ident: &Ident, variant_ident: &Ident, fields: FieldsUnnamed) -> TokenStream2 {
    todo!()
}

fn named(enum_ident: &Ident, variant_ident: &Ident, fields: FieldsNamed) -> TokenStream2 {
    todo!()
}
