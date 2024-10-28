use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DataEnum, FieldsNamed, FieldsUnnamed, Ident};

pub fn derive(data_enum: DataEnum) -> TokenStream2 {
    // TODO: note that this this must take future 'skip', 'rename' and 'flatten'
    // attributes into account too.
    let not_found_error = {
        let variant_names = data_enum.variants.iter().map(|variant| variant.ident.to_string());
        quote! { Err(::ximple::de::Error::element_not_found([#(#variant_names),*].map(ToString::to_string))) }
    };

    let variant_deserializers = data_enum.variants.into_iter().map(|variant| match variant.fields {
        syn::Fields::Named(fields_named) => named(&variant.ident, fields_named),
        syn::Fields::Unnamed(fields_unnamed) => unnamed(&variant.ident, fields_unnamed),
        syn::Fields::Unit => unit(&variant.ident),
    });

    quote! {
        #(#variant_deserializers)*
        #not_found_error
    }
}

fn unit(variant_ident: &Ident) -> TokenStream2 {
    let variant_name_variable = utils::variant_name_variable(variant_ident);

    quote! {
        #variant_name_variable
        if deserializer.peek_start_element_matches(variant_name) {
            deserializer.read_element::<()>(variant_name)?;
            return Ok(Self::#variant_ident);
        }
    }
}

fn unnamed(variant_ident: &Ident, fields: FieldsUnnamed) -> TokenStream2 {
    let variant_name_variable = utils::variant_name_variable(variant_ident);

    let unnamed_field_deserializers = fields.unnamed.iter().map(|_| {
        quote! { ::ximple::FromXml::deserialize(deserializer)? }
    });

    quote! {
        #variant_name_variable
        if deserializer.peek_start_element_matches(variant_name) {
            deserializer.read_start_element::<()>(variant_name)?;

            let value = Self::#variant_ident(#(#unnamed_field_deserializers),*);

            deserializer.read_end_element(variant_name)?;

            return Ok(value);
        }
    }
}

fn named(variant_ident: &Ident, fields: FieldsNamed) -> TokenStream2 {
    let variant_name_variable = utils::variant_name_variable(variant_ident);

    let named_field_deserializers = fields.named.into_iter().map(|field| {
        let field_ident = field.ident.expect("encountered named fields without identifier");
        let field_ident_string = field_ident.to_string();
        quote! {
            #field_ident: deserializer.read_element(::ximple::types::Name::new(#field_ident_string))?
        }
    });

    quote! {
        #variant_name_variable
        if deserializer.peek_start_element_matches(variant_name) {
            deserializer.read_start_element::<()>(variant_name)?;

            let value = Self::#variant_ident {
                #(#named_field_deserializers),*
            };

            deserializer.read_end_element(variant_name)?;

            return Ok(value);
        }
    }
}

mod utils {
    use super::*;

    pub fn variant_name_variable(variant_ident: &Ident) -> TokenStream2 {
        // TODO: take prefix and rename attributes into account
        let variant_name = variant_ident.to_string();

        quote! {
            let variant_name = ::ximple::types::Name::new(#variant_name);
        }
    }
}
