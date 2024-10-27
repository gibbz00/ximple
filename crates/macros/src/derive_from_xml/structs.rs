use proc_macro2::TokenStream as TokenStream2;
use syn::{DataStruct, Fields, FieldsNamed, FieldsUnnamed, Ident};

pub fn derive(ident: Ident, data_struct: DataStruct) -> TokenStream2 {
    match data_struct.fields {
        Fields::Unit => unit(ident),
        Fields::Named(fields_named) => named(ident, fields_named),
        Fields::Unnamed(fields_unnamed) => unnamed(ident, fields_unnamed),
    }
}

fn unit(ident: Ident) -> TokenStream2 {
    todo!()
}

fn named(ident: Ident, fields: FieldsNamed) -> TokenStream2 {
    todo!()
}

fn unnamed(ident: Ident, fields: FieldsUnnamed) -> TokenStream2 {
    todo!()
}
