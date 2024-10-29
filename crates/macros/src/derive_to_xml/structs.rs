use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Attribute, DataStruct, Fields, FieldsNamed};

pub fn derive(container_attributes: Vec<Attribute>, data_struct: DataStruct) -> syn::Result<TokenStream2> {
    match data_struct.fields {
        Fields::Unit => unit(),
        Fields::Named(fields_named) => named(fields_named),
        Fields::Unnamed(fields_unnamed) => unnamed::unnamed(container_attributes, fields_unnamed),
    }
}

fn unit() -> syn::Result<TokenStream2> {
    Ok(quote! {
        Ok(())
    })
}

fn named(fields: FieldsNamed) -> syn::Result<TokenStream2> {
    let elements = fields.named.into_iter().map(|field| {
        let field_ident = field.ident.expect("encountered named field without an identifier");
        let element_name = field_ident.to_string();

        quote! {
            serializer.write_element(#element_name, &self.#field_ident)?;
        }
    });

    Ok(quote! {
        #(#elements)*
        Ok(())
    })
}

mod unnamed {
    use proc_macro2::TokenStream as TokenStream2;
    use quote::quote;
    use syn::{Attribute, FieldsUnnamed};

    use crate::*;

    pub fn unnamed(container_attributes: Vec<Attribute>, fields: FieldsUnnamed) -> syn::Result<TokenStream2> {
        let _container_attribute_set = AttributeParser::parse::<TupleContainerAttribute>(container_attributes, ())?;

        let elements = fields.unnamed.into_iter().enumerate().map(|(index, _)| {
            let index = syn::Index::from(index);
            quote! {
                self.#index.serialize(serializer)?;
            }
        });

        Ok(quote! {
            #(#elements)*
            Ok(())
        })
    }

    pub(crate) use attributes::TupleContainerAttribute;
    mod attributes {
        use crate::*;

        pub enum TupleContainerAttribute {
            // TODO: serialize_with
        }

        pub enum TupleContainerAttributeSet {
            None,
        }

        impl XimpleAttribute for TupleContainerAttribute {
            type AttributeSet = TupleContainerAttributeSet;
            type CompatibilityContext = ();

            fn name(&self) -> &'static str {
                unreachable!("no tuple strucs attributes have been defined")
            }

            fn parse_attribute(nested_meta: syn::meta::ParseNestedMeta<'_>) -> Result<Self, AttributeParseError> {
                Err(AttributeParseError::unknown(nested_meta.input.span()))
            }

            fn into_compatible_set(
                _buffer: AttributeBuffer<Self>,
                _context: Self::CompatibilityContext,
            ) -> Result<Self::AttributeSet, AttributeParseError> {
                Ok(TupleContainerAttributeSet::None)
            }
        }
    }
}
