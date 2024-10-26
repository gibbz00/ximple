mod structs;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput};

pub fn derive_to_xml_impl(token_stream: TokenStream) -> TokenStream {
    let type_definition = parse_macro_input!(token_stream as DeriveInput);

    let ident = type_definition.ident;

    match type_definition.data {
        Data::Struct(data_struct) => structs::derive(ident, data_struct),
        Data::Enum(data_enum) => derive_enum(data_enum),
        Data::Union(data_union) => syn::Error::new(data_union.union_token.span, "unions are not supported").to_compile_error(),
    }
    .into()
}

fn derive_enum(data_enum: DataEnum) -> TokenStream2 {
    todo!()
}
