use darling::ast::Data;
use darling::FromDeriveInput;
use quote::{quote, ToTokens};

use crate::common::enums::EnumData;
use crate::common::Codegen;

mod fields;

#[derive(Debug, FromDeriveInput)]
#[darling(
    forward_attrs(into),
    supports(struct_named, enum_unit, enum_named, enum_newtype),
    and_then = "Self::validate"
)]
pub(crate) struct IntoInputReceiver {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<EnumData<fields::FieldsData>, fields::FieldsData>,

    attrs: Vec<syn::Attribute>,

    #[darling(skip)]
    type_: Option<syn::Path>,
}

impl IntoInputReceiver {
    fn validate(mut self) -> darling::Result<Self> {
        let attr_meta = match self.attrs.as_slice() {
            [attr] => &attr.meta,
            _ => {
                return Err(darling::Error::custom(
                    r#"Missing attribute '#[into(...)]'"#,
                ))
            }
        };

        let tokens = match attr_meta {
            syn::Meta::List(ml) => &ml.tokens,
            _ => return Err(darling::Error::custom("Invalid 'into' attribute format")),
        };

        if let Ok(path) = syn::parse(tokens.to_owned().into()) {
            self.type_ = Some(path);
        } else if let Ok(path_string) = syn::parse::<syn::LitStr>(tokens.to_owned().into()) {
            self.type_ = Some(path_string.parse()?)
        }

        Ok(self)
    }
}

impl ToTokens for IntoInputReceiver {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let IntoInputReceiver {
            ident,
            generics,
            data,
            type_,
            ..
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
                let names = struct_data.iter().flat_map(|entry| entry.generate_names());
                let conversions = struct_data
                    .iter()
                    .flat_map(|entry| entry.generate_conversion());

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
