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

mod structs {
    use proc_macro2::TokenStream as TokenStream2;
    use quote::quote;
    use syn::{DataStruct, Fields, FieldsNamed, Ident};

    pub fn derive(ident: Ident, data_struct: DataStruct) -> TokenStream2 {
        match data_struct.fields {
            Fields::Named(fields_named) => named(ident, fields_named),
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
}

fn derive_enum(data_enum: DataEnum) -> TokenStream2 {
    todo!()
}
