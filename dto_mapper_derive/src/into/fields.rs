use darling::util::Flag;
use darling::FromField;
use quote::{format_ident, quote, ToTokens};

use crate::common::fields::{generate_field_conversion, generate_unwrap_or};
use crate::common::Codegen;

#[derive(Debug, FromField)]
#[darling(attributes(into), and_then = "Self::validate")]
pub(crate) struct FieldsData {
    ident: Option<syn::Ident>,
    skip: Flag,
    unwrap_or: Option<syn::Expr>,
    with_fn: Option<syn::Path>,
    with_fn_ref: Option<syn::Path>,
    rename: Option<syn::Ident>,
}

impl FieldsData {
    fn validate(self) -> darling::Result<Self> {
        if self.skip.is_present()
            && (self.with_fn.is_some()
                || self.with_fn_ref.is_some()
                || self.unwrap_or.is_some()
                || self.rename.is_some())
        {
            return Err(darling::Error::custom(
                "`skip` cannot be used with other attributes",
            ));
        }

        if self.unwrap_or.is_some() && (self.with_fn.is_some() || self.with_fn_ref.is_some()) {
            return Err(darling::Error::custom(
                "`unwrap_or` cannot be used together with `with_fn` or `with_fn_ref`",
            ));
        }

        if self.with_fn.is_some() && self.with_fn_ref.is_some() {
            return Err(darling::Error::custom(
                "Only one can be used: (with_fn, with_fn_ref)",
            ));
        }

        Ok(self)
    }
}

impl Codegen for FieldsData {
    fn generate_conversion(&self) -> Option<proc_macro2::TokenStream> {
        if self.skip.is_present() {
            return None;
        }

        let left_field_name = self.rename.as_ref().or(self.ident.as_ref()).unwrap();
        let right_field_name = self.ident.as_ref().unwrap().to_token_stream();

        let conversion = if let Some(unwrap_or) = &self.unwrap_or {
            generate_unwrap_or(&right_field_name, unwrap_or)
        } else {
            generate_field_conversion(&right_field_name, &self.with_fn, &self.with_fn_ref)
        };

        Some(quote!(#left_field_name: #conversion))
    }

    fn generate_conversion_for_ident(&self, ident: &str) -> Option<proc_macro2::TokenStream> {
        if self.skip.is_present() {
            return None;
        }

        let ident = format_ident!("{ident}").to_token_stream();

        if let Some(unwrap_or) = &self.unwrap_or {
            generate_unwrap_or(&ident, unwrap_or)
        } else {
            generate_field_conversion(&ident, &self.with_fn, &self.with_fn_ref)
        }
        .into()
    }

    fn generate_names(&self) -> Option<proc_macro2::TokenStream> {
        if self.skip.is_present() {
            return None;
        }

        Some(self.ident.to_token_stream())
    }
}
