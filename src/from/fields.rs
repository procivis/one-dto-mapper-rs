use darling::util::Flag;
use darling::FromField;
use quote::{format_ident, quote, ToTokens};

use super::Conversion;
use crate::common::fields::{generate_field_conversion, generate_unwrap_or};
use crate::common::{Codegen, Context};

#[derive(Debug, FromField)]
#[darling(attributes(convert), and_then = "Self::validate")]
pub(crate) struct FieldsData {
    ident: Option<syn::Ident>,
    skip: Flag,
    unwrap_or: Option<syn::Expr>,
    into_unwrap_or: Option<syn::Expr>,
    from_unwrap_or: Option<syn::Expr>,
    with_fn: Option<syn::Path>,
    with_fn_ref: Option<syn::Path>,
    rename: Option<syn::Ident>,
    replace: Option<syn::Expr>,
}

impl FieldsData {
    fn validate(self) -> darling::Result<Self> {
        if self.with_fn.is_some() && self.with_fn_ref.is_some() {
            return Err(darling::Error::custom(
                "Only one can be used: (with_fn, with_fn_ref)",
            ));
        }

        if (self.unwrap_or.is_some()
            || self.into_unwrap_or.is_some()
            || self.from_unwrap_or.is_some())
            && (self.with_fn.is_some() || self.with_fn_ref.is_some())
        {
            return Err(darling::Error::custom(
                "`unwrap_or`, `into_unwrap_or` and `from_unwrap_or` cannot be used together with `with_fn` or `with_fn_ref`",
            ));
        }

        if self.unwrap_or.is_some()
            && (self.into_unwrap_or.is_some() || self.from_unwrap_or.is_some())
        {
            return Err(darling::Error::custom(
                "`unwrap_or` cannot be used with `into_unwrap_or` or `from_unwrap_or`",
            ));
        }

        Ok(self)
    }

    pub(crate) fn validate_with_conversions(
        &self,
        conversions: &[Conversion],
    ) -> darling::Result<()> {
        if conversions.len() != 1 && self.unwrap_or.is_some() {
            return Err(darling::Error::custom(
                "`unwrap_or` can be used only with a single `from` or `into`. \
                If you need both `from` and `into` use `into_unwrap_or` or `from_unwrap_or`",
            ));
        }

        if !conversions.contains(&Conversion::From) && self.from_unwrap_or.is_some() {
            return Err(darling::Error::custom(
                "`from_unwrap_or` can be used only when `from` is specified",
            ));
        }

        if !conversions.contains(&Conversion::Into) && self.into_unwrap_or.is_some() {
            return Err(darling::Error::custom(
                "`into_unwrap_or` can be used only when `into` is specified",
            ));
        }

        if self.unwrap_or.is_some()
            && (self.into_unwrap_or.is_some() || self.from_unwrap_or.is_some())
        {
            return Err(darling::Error::custom(
                "`unwrap_or` cannot be used with `into_unwrap_or` or `from_unwrap_or`",
            ));
        }

        Ok(())
    }
}

impl Codegen for FieldsData {
    fn generate_conversion(&self, context: &Context) -> Option<proc_macro2::TokenStream> {
        if matches!(context.conversion, Conversion::Into) && self.skip.is_present() {
            return None;
        }

        let left_field_name = match context.conversion {
            Conversion::Into => self.rename.as_ref().or(self.ident.as_ref()),
            Conversion::From => self.ident.as_ref(),
        }
        .unwrap();

        let right_field_name = match context.conversion {
            Conversion::Into => self.ident.as_ref(),
            Conversion::From => self.rename.as_ref().or(self.ident.as_ref()),
        }
        .unwrap()
        .to_token_stream();

        let unwrap_or = self.unwrap_or.as_ref().or(match context.conversion {
            Conversion::Into => self.into_unwrap_or.as_ref(),
            Conversion::From => self.from_unwrap_or.as_ref(),
        });

        let conversion = if let Some(unwrap_or) = unwrap_or {
            generate_unwrap_or(&right_field_name, unwrap_or)
        } else if let Some(expr) = &self.replace {
            quote!(#expr.into())
        } else {
            generate_field_conversion(&right_field_name, &self.with_fn, &self.with_fn_ref)
        };

        Some(quote!(#left_field_name: #conversion))
    }

    fn generate_conversion_for_ident(
        &self,
        context: &Context,
        ident: &str,
    ) -> Option<proc_macro2::TokenStream> {
        if matches!(context.conversion, Conversion::Into) && self.skip.is_present() {
            return None;
        }

        let ident = format_ident!("{ident}").to_token_stream();

        let unwrap_or = self.unwrap_or.as_ref().or(match context.conversion {
            Conversion::Into => self.into_unwrap_or.as_ref(),
            Conversion::From => self.from_unwrap_or.as_ref(),
        });

        Some(if let Some(unwrap_or) = unwrap_or {
            generate_unwrap_or(&ident, unwrap_or)
        } else if let Some(expr) = &self.replace {
            quote!(#expr.into())
        } else {
            generate_field_conversion(&ident, &self.with_fn, &self.with_fn_ref)
        })
    }

    fn generate_names(&self, context: &Context) -> Option<proc_macro2::TokenStream> {
        if matches!(context.conversion, Conversion::Into) && self.skip.is_present() {
            return None;
        }

        if matches!(context.conversion, Conversion::From) && self.replace.is_some() {
            return None;
        }

        match context.conversion {
            Conversion::Into => self.ident.to_token_stream(),
            Conversion::From => self
                .rename
                .as_ref()
                .or(self.ident.as_ref())
                .to_token_stream(),
        }
        .into()
    }
}
