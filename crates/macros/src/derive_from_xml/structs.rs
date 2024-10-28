use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DataStruct, Fields, FieldsNamed, FieldsUnnamed};

pub fn derive(data_struct: DataStruct) -> TokenStream2 {
    match data_struct.fields {
        Fields::Unit => unit(),
        Fields::Named(fields_named) => named(fields_named),
        Fields::Unnamed(fields_unnamed) => unnamed(fields_unnamed),
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

fn unnamed(fields: FieldsUnnamed) -> TokenStream2 {
    let tuple_fields = fields.unnamed.into_iter().map(|_| {
        quote! {
            ::ximple::FromXml::deserialize(deserializer)?
        }
    });

    quote! {
        Ok(Self(#(#tuple_fields),*))
    }
}
