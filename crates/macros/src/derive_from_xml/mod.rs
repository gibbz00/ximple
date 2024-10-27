mod enums;
mod structs;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

pub fn derive_from_xml_impl(token_stream: TokenStream) -> TokenStream {
    let type_definition = parse_macro_input!(token_stream as DeriveInput);

    let ident = type_definition.ident;

    match type_definition.data {
        Data::Struct(data_struct) => structs::derive(ident, data_struct),
        Data::Enum(data_enum) => enums::derive(ident, data_enum),
        Data::Union(data_union) => syn::Error::new(data_union.union_token.span, "unions are not supported").to_compile_error(),
    }
    .into()
}
