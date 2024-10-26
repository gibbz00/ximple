use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

pub fn derive_to_xml_impl(token_stream: TokenStream) -> TokenStream {
    let type_definition = parse_macro_input!(token_stream as DeriveInput);

    match type_definition.data {
        Data::Struct(data_struct) => todo!(),
        Data::Enum(data_enum) => todo!(),
        Data::Union(data_union) => {
            return syn::Error::new(data_union.union_token.span, "unions are not supported")
                .to_compile_error()
                .into()
        }
    }

    todo!()
}
