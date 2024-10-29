use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DataStruct, Fields, FieldsNamed};

pub fn derive_from_struct(data_struct: DataStruct) -> TokenStream2 {
    match data_struct.fields {
        Fields::Unit => unit(),
        Fields::Named(fields_named) => named(fields_named),
        Fields::Unnamed(fields_unnamed) => unnamed::unnamed(fields_unnamed),
    }
}

fn unit() -> TokenStream2 {
    quote! {
        Ok(Self)
    }
}

fn named(fields: FieldsNamed) -> TokenStream2 {
    let named_fields = fields.named.into_iter().map(|field| {
        let field_ident = field.ident.expect("encountered named field without an identifier");
        let element_ident = field_ident.to_string();
        let element_name = quote! { ::ximple::types::Name::new(#element_ident) };

        quote! {
            #field_ident: deserializer.read_element(#element_name)?
        }
    });

    quote! {
        Ok(Self{
            #(#named_fields),*
        })
    }
}

pub(crate) use unnamed::FromXmlStructUnnamedContainerAttribute;
mod unnamed {
    use proc_macro2::TokenStream as TokenStream2;
    use quote::quote;
    use syn::FieldsUnnamed;

    pub fn unnamed(fields: FieldsUnnamed) -> TokenStream2 {
        let tuple_fields = fields.unnamed.into_iter().map(|_| {
            quote! {
                ::ximple::FromXml::deserialize(deserializer)?
            }
        });

        quote! {
            Ok(Self(#(#tuple_fields),*))
        }
    }

    pub(crate) use attributes::FromXmlStructUnnamedContainerAttribute;
    mod attributes {
        use syn::Ident;

        use crate::*;

        pub enum FromXmlStructUnnamedContainerAttribute {
            // TODO: deserialize_with
        }

        pub enum TupleContainerAttributeSet {
            None,
        }

        impl XimpleAttribute for FromXmlStructUnnamedContainerAttribute {
            type AttributeSet = TupleContainerAttributeSet;
            type CompatibilityContext = ();

            const DERIVE_NAME: DeriveName = DeriveName::FromXml;

            fn name(&self) -> &'static str {
                unreachable!("no tuple strucs attributes have been defined")
            }

            fn is_name(_: &Ident) -> bool {
                false
            }

            fn parse_attribute(nested_meta: syn::meta::ParseNestedMeta<'_>) -> Result<Self, AttributeParseError> {
                Err(AttributeParseError::unknown(nested_meta.input.span()))
            }

            fn into_compatible_set(
                _buffer: AttributeBuffer<Self>,
                _context: Self::CompatibilityContext,
            ) -> Result<Self::AttributeSet, AttributeParseError> {
                Ok(TupleContainerAttributeSet::None)
            }
        }
    }
}
