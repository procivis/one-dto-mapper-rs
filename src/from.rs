use darling::ast::Data;
use darling::FromDeriveInput;
use itertools::Either;
use quote::{quote, ToTokens};

use crate::common::enums::EnumData;
use crate::common::{Codegen, Context, Conversion};

mod fields;

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(convert),
    supports(struct_named, enum_unit, enum_named, enum_newtype),
    and_then = "Self::validate_from_or_into"
)]
pub(crate) struct FromInputReceiver {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<EnumData<fields::FieldsData>, fields::FieldsData>,
    /// Attribute #[convert(into = "...")]
    into: Option<syn::Path>,
    /// Attribute #[convert(from = "...")]
    from: Option<syn::Path>,
}

impl FromInputReceiver {
    fn validate_from_or_into(self) -> darling::Result<Self> {
        if self.from.is_none() && self.into.is_none() {
            return Err(darling::Error::custom("Missing one of the attributes '#[convert(from = \"...\")]' or '#[convert(into = \"...\")]'"));
        }

        let conversions: Vec<_> = [
            self.from.as_ref().map(|_| Conversion::From),
            self.into.as_ref().map(|_| Conversion::Into),
        ]
        .into_iter()
        .flatten()
        .collect();

        match &self.data {
            Data::Enum(fields) => Either::Left(fields.iter().flat_map(|entry| entry.fields.iter())),
            Data::Struct(fields) => Either::Right(fields.fields.iter()),
        }
        .try_for_each(|field| field.validate_with_conversions(&conversions))?;

        Ok(self)
    }
}

impl ToTokens for FromInputReceiver {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let FromInputReceiver {
            ident,
            generics,
            data,
            from,
            into,
        } = self;

        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

        // it's guaranteed from `and_then = "Self::validate_from_or_into"` that there will be at least one of the attributes present
        let from_into_attributes = [
            from.as_ref().map(|from_ident| {
                (
                    quote!(#from_ident),
                    quote!(#ident #type_generics),
                    Conversion::From,
                )
            }),
            into.as_ref().map(|into_ident| {
                (
                    quote!(#ident #type_generics),
                    quote!(#into_ident),
                    Conversion::Into,
                )
            }),
        ];

        for (from_ident, into_ident, conversion) in from_into_attributes.into_iter().flatten() {
            let context = Context {
                from_ident: from_ident.clone(),
                conversion,
            };

            let body = match data {
                Data::Enum(enum_data) => {
                    let entries = enum_data.iter().map(|entry| entry.generate(&context));
                    quote! {
                        match __value {
                            #(#entries),*
                        }
                    }
                }
                Data::Struct(struct_data) => {
                    let names = struct_data
                        .iter()
                        .flat_map(|entry| entry.generate_names(&context));
                    let conversions = struct_data
                        .iter()
                        .flat_map(|entry| entry.generate_conversion(&context));

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
                impl #impl_generics From<#from_ident> for #into_ident #where_clause {
                    fn from(__value: #from_ident) -> Self {
                        #body
                    }
                }
            })
        }
    }
}
