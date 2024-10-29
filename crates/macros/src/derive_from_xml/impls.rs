use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, TypeParamBound};

use crate::*;

pub fn derive_from_xml_impl(token_stream: TokenStream) -> TokenStream {
    let type_definition = parse_macro_input!(token_stream as DeriveInput);

    let ident = type_definition.ident;

    let impl_body = match type_definition.data {
        Data::Struct(data_struct) => derive_from_struct(data_struct),
        Data::Enum(data_enum) => derive_from_enum(data_enum),
        Data::Union(data_union) => syn::Error::new(data_union.union_token.span, "unions are not supported").to_compile_error(),
    };

    let from_xml_bounds: TypeParamBound = syn::parse_str("::ximple::FromXml").expect("valid type parameter bound");
    let mut generics = type_definition.generics;
    generics
        .type_params_mut()
        .for_each(|param| param.bounds.push(from_xml_bounds.clone()));
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::ximple::FromXml for #ident #type_generics #where_clause {
            fn deserialize(deserializer: &mut ::ximple::de::Deserializer<impl std::io::Read>) -> Result<Self, ::ximple::de::Error> {
                #impl_body
            }
        }
    }
    .into()
}
