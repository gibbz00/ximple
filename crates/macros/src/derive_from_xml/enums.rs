use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DataEnum, FieldsNamed, FieldsUnnamed, Ident};

pub fn derive(enum_ident: Ident, data_enum: DataEnum) -> TokenStream2 {
    let match_arms = data_enum.variants.into_iter().map(|variant| match variant.fields {
        syn::Fields::Named(fields_named) => named(&enum_ident, &variant.ident, fields_named),
        syn::Fields::Unnamed(fields_unnamed) => unnamed(&enum_ident, &variant.ident, fields_unnamed),
        syn::Fields::Unit => unit(&enum_ident, &variant.ident),
    });

    quote! {
        impl ::ximple::FromXml for #enum_ident {
            fn deserialize(deserializer: &mut ::ximple::de::Deserializer<impl std::io::Read>) -> Result<Self, ::ximple::de::Error> {
                match self {
                    #(#match_arms),*
                }
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
