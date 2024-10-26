use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{DataEnum, FieldsNamed, FieldsUnnamed, Ident};

pub fn derive(enum_ident: Ident, data_enum: DataEnum) -> TokenStream2 {
    let match_arms = data_enum.variants.into_iter().map(|variant| match variant.fields {
        syn::Fields::Named(fields_named) => named(&enum_ident, &variant.ident, fields_named),
        syn::Fields::Unnamed(fields_unnamed) => unnamed(&enum_ident, &variant.ident, fields_unnamed),
        syn::Fields::Unit => unit(&enum_ident, &variant.ident),
    });

    quote! {
        impl ::ximple::ToXml for #enum_ident {
            fn serialize(&self, serializer: &mut ::ximple::ser::Serializer<impl std::io::Write>) -> Result<(), ::ximple::ser::Error> {
                match self {
                    #(#match_arms),*
                }
            }
        }
    }
}

fn unit(enum_ident: &Ident, variant_ident: &Ident) -> TokenStream2 {
    let element_name = variant_ident.to_string();
    quote! {
        #enum_ident::#variant_ident => serializer.write_element(#element_name, &())
    }
}

fn unnamed(enum_ident: &Ident, variant_ident: &Ident, fields: FieldsUnnamed) -> TokenStream2 {
    let element_name = variant_ident.to_string();

    let field_placeholders = fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(index, _)| syn::Ident::new(&format!("p{}", index), Span::call_site()));

    let field_serialization = field_placeholders.clone().map(|placeholder| {
        quote! {
            #placeholder.serialize(serializer)?;
        }
    });

    quote! {
        #enum_ident::#variant_ident(#(#field_placeholders),*) => {
            serializer.write_start(#element_name)?;
            #(#field_serialization)*
            serializer.write_end()
        }
    }
}

fn named(enum_ident: &Ident, variant_ident: &Ident, fields: FieldsNamed) -> TokenStream2 {
    let element_name = variant_ident.to_string();

    let field_idents = fields.named.iter().map(|field| &field.ident);

    let field_serialization = fields.named.iter().map(|field| {
        let field_ident = field.ident.as_ref().expect("encountered a named field without an identifier");
        let field_element_name = field_ident.to_string();

        quote! {
            serializer.write_element(#field_element_name, #field_ident)?;
        }
    });

    quote! {
        #enum_ident::#variant_ident { #(#field_idents),* } => {
            serializer.write_start(#element_name)?;
            #(#field_serialization)*
            serializer.write_end()
        }
    }
}
