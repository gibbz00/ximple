use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DataStruct, Fields, FieldsNamed, FieldsUnnamed, Ident};

pub fn derive(ident: Ident, data_struct: DataStruct) -> TokenStream2 {
    match data_struct.fields {
        Fields::Unit => unit(ident),
        Fields::Named(fields_named) => named(ident, fields_named),
        Fields::Unnamed(fields_unnamed) => unnamed(ident, fields_unnamed),
    }
}

fn unit(ident: Ident) -> TokenStream2 {
    quote! {
        impl ::ximple::FromXml for #ident {
            fn deserialize(deserializer: &mut ::ximple::de::Deserializer<impl std::io::Read>) -> Result<Self, ::ximple::de::Error> {
                Ok(#ident)
            }
        }
    }
}

fn named(ident: Ident, fields: FieldsNamed) -> TokenStream2 {
    let named_fields = fields.named.into_iter().map(|field| {
        let field_ident = field.ident.expect("encountered named field without an identifier");
        let element_ident = field_ident.to_string();
        let element_name = quote! { ::ximple::types::Name::new(#element_ident) };

        quote! {
            #field_ident: deserializer.read_element(#element_name)?
        }
    });

    quote! {
        impl ::ximple::FromXml for #ident {
            fn deserialize(deserializer: &mut ::ximple::de::Deserializer<impl std::io::Read>) -> Result<Self, ::ximple::de::Error> {
                Ok(#ident{
                    #(#named_fields),*
                })
            }
        }
    }
}

fn unnamed(ident: Ident, fields: FieldsUnnamed) -> TokenStream2 {
    let tuple_fields = fields.unnamed.into_iter().map(|field| {
        let ty = field.ty;
        quote! {
            #ty::deserialize(deserializer)?
        }
    });

    quote! {
        impl ::ximple::FromXml for #ident {
            fn deserialize(deserializer: &mut ::ximple::de::Deserializer<impl std::io::Read>) -> Result<Self, ::ximple::de::Error> {
                Ok(#ident(#(#tuple_fields),*))
            }
        }
    }
}
