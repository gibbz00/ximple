use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields, Ident};

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

mod structs {

    use quote::quote;

    use super::*;

    pub fn derive(ident: Ident, data_struct: DataStruct) -> TokenStream2 {
        match data_struct.fields {
            Fields::Named(fields_named) => todo!(),
            Fields::Unnamed(fields_unnamed) => todo!(),
            Fields::Unit => unit(ident),
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
}

fn derive_enum(data_enum: DataEnum) -> TokenStream2 {
    todo!()
}
