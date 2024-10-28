mod enums;
mod structs;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, TypeParamBound};

pub fn derive_to_xml_impl(token_stream: TokenStream) -> TokenStream {
    let type_definition = parse_macro_input!(token_stream as DeriveInput);

    let ident = type_definition.ident;
    let container_attributes = type_definition.attrs;

    let impl_body = match type_definition.data {
        Data::Struct(data_struct) => structs::derive(data_struct),
        Data::Enum(data_enum) => enums::derive(&ident, data_enum),
        Data::Union(data_union) => syn::Error::new(data_union.union_token.span, "unions are not supported").to_compile_error(),
    };

    let to_xml_bounds: TypeParamBound = syn::parse_str("::ximple::ToXml").expect("valid type parameter bound");
    let mut generics = type_definition.generics;
    generics
        .type_params_mut()
        .for_each(|param| param.bounds.push(to_xml_bounds.clone()));
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::ximple::ToXml for #ident #type_generics #where_clause {
            fn serialize(&self, serializer: &mut ::ximple::ser::Serializer<impl std::io::Write>) -> Result<(), ::ximple::ser::Error> {
                #impl_body
            }
        }
    }
    .into()
}
