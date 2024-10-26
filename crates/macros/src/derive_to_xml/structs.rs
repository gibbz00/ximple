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
        impl ::ximple::ToXml for #ident {
            fn serialize(&self, serializer: &mut ::ximple::ser::Serializer<impl std::io::Write>) -> Result<(), ::ximple::ser::Error> {
                Ok(())
            }
        }
    }
}

fn named(ident: Ident, fields: FieldsNamed) -> TokenStream2 {
    let elements = fields.named.into_iter().map(|field| {
        let field_ident = field.ident.expect("encountered named field without an identifier");
        let element_name = field_ident.to_string();

        quote! {
            serializer.write_element(#element_name, &self.#field_ident)?;
        }
    });

    quote! {
        impl ::ximple::ToXml for #ident {
            fn serialize(&self, serializer: &mut ::ximple::ser::Serializer<impl std::io::Write>) -> Result<(), ::ximple::ser::Error> {
                #(#elements)*
                Ok(())
            }
        }
    }
}

fn unnamed(ident: Ident, fields: FieldsUnnamed) -> TokenStream2 {
    let elements = fields.unnamed.into_iter().enumerate().map(|(index, _)| {
        let index = syn::Index::from(index);
        quote! {
            self.#index.serialize(serializer)?;
        }
    });

    quote! {
        impl ::ximple::ToXml for #ident {
            fn serialize(&self, serializer: &mut ::ximple::ser::Serializer<impl std::io::Write>) -> Result<(), ::ximple::ser::Error> {
                #(#elements)*
                Ok(())
            }
        }
    }
}
