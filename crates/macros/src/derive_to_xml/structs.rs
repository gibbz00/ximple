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
        Ok(())
    }
}

fn named(fields: FieldsNamed) -> TokenStream2 {
    let elements = fields.named.into_iter().map(|field| {
        let field_ident = field.ident.expect("encountered named field without an identifier");
        let element_name = field_ident.to_string();

        quote! {
            serializer.write_element(#element_name, &self.#field_ident)?;
        }
    });

    quote! {
        #(#elements)*
        Ok(())
    }
}

fn unnamed(fields: FieldsUnnamed) -> TokenStream2 {
    let elements = fields.unnamed.into_iter().enumerate().map(|(index, _)| {
        let index = syn::Index::from(index);
        quote! {
            self.#index.serialize(serializer)?;
        }
    });

    quote! {
        #(#elements)*
        Ok(())
    }
}
