use darling::ast::Data;
use darling::FromDeriveInput;
use quote::{quote, ToTokens};

use crate::common::enums::EnumData;
use crate::common::Codegen;

mod fields;

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(try_into),
    supports(struct_named, enum_unit, enum_named, enum_newtype),
    and_then = "Self::validate"
)]
pub(crate) struct TryIntoInputReceiver {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<EnumData<fields::FieldsData>, fields::FieldsData>,

    #[darling(rename = "T")]
    type_: Option<syn::Path>,
    #[darling(rename = "Error")]
    error: Option<syn::Path>,
}

impl TryIntoInputReceiver {
    fn validate(self) -> darling::Result<Self> {
        if self.type_.is_none() {
            return Err(darling::Error::custom(
                r#"Missing attribute '#[try_into(T = "...")]'"#,
            ));
        }

        if self.error.is_none() {
            return Err(darling::Error::custom(
                r#"Missing attribute '#[try_into(Error = "...")]'"#,
            ));
        }

        Ok(self)
    }
}

impl ToTokens for TryIntoInputReceiver {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let TryIntoInputReceiver {
            ident,
            generics,
            data,
            type_,
            error,
        } = self;

        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

        let into_ident = quote!(#type_);
        let from_ident = quote!(#ident #type_generics);

        let body = match data {
            Data::Enum(enum_data) => {
                let entries = enum_data
                    .iter()
                    .map(|entry| entry.generate(from_ident.clone()));
                quote! {
                    match __value {
                        #(#entries),*
                    }
                }
            }
            Data::Struct(struct_data) => {
                let names = struct_data
                    .iter()
                    .filter_map(|entry| entry.generate_names());
                let conversions = struct_data
                    .iter()
                    .filter_map(|entry| entry.generate_conversion());

                quote! {
                    match __value {
                        #from_ident {
                            #(#names),*
                            ,..
                        } => Self {
                            #(#conversions),*
                        }
                    }
                }
            }
        };

        tokens.extend(quote! {
            impl #impl_generics TryFrom<#from_ident> for #into_ident #where_clause {
                type Error = #error;

                fn try_from(__value: #from_ident) -> Result<Self, Self::Error> {
                    Ok(#body)
                }
            }
        })
    }
}
