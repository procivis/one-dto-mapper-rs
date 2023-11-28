use darling::util::Flag;
use darling::FromField;
use quote::quote;

use super::Conversion;

#[derive(Debug, FromField)]
#[darling(attributes(convert), and_then = "Self::validate")]
pub(crate) struct FieldsData {
    pub(crate) ident: Option<syn::Ident>,
    pub(crate) skip: Flag,
    pub(crate) unwrap_or: Option<syn::Expr>,
    pub(crate) into_unwrap_or: Option<syn::Expr>,
    pub(crate) from_unwrap_or: Option<syn::Expr>,
    pub(crate) with_fn: Option<syn::Path>,
    pub(crate) with_fn_ref: Option<syn::Path>,
    pub(crate) rename: Option<syn::Ident>,
    pub(crate) replace: Option<syn::Expr>,
}

impl FieldsData {
    fn validate(self) -> darling::Result<Self> {
        if self.with_fn.is_some() && self.with_fn_ref.is_some() {
            return Err(darling::Error::custom(
                "Only one can be used: (with_fn,with_fn_ref)",
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

    pub(crate) fn validate_with_directions(
        &self,
        directions: &[Conversion],
    ) -> darling::Result<()> {
        if directions.len() != 1 && self.unwrap_or.is_some() {
            return Err(darling::Error::custom(
                "`unwrap_or` can be used only with a single `from` or `into`. \
                If you need both `from` and `into` use `into_unwrap_or` or `from_unwrap_or`",
            ));
        }

        if !directions.contains(&Conversion::From) && self.from_unwrap_or.is_some() {
            return Err(darling::Error::custom(
                "`from_unwrap_or` can be used only when `from` is specified",
            ));
        }

        if !directions.contains(&Conversion::Into) && self.into_unwrap_or.is_some() {
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

pub(super) fn generate_unwrap_or(
    field_path: &proc_macro2::TokenStream,
    unwrap_or: &syn::Expr,
) -> proc_macro2::TokenStream {
    quote!(#field_path.unwrap_or(#unwrap_or))
}

pub(super) fn generate_field_conversion(
    field_path: &proc_macro2::TokenStream,
    with_fn: &Option<syn::Path>,
    with_fn_ref: &Option<syn::Path>,
) -> proc_macro2::TokenStream {
    if let Some(function) = with_fn {
        return quote!(#function(#field_path));
    }

    if let Some(function) = with_fn_ref {
        return quote!(#function(&#field_path));
    }

    quote!(#field_path.into())
}

#[cfg(test)]
mod tests {
    use darling::FromMeta;

    use super::*;

    #[test]
    fn test_into_field_conversion() {
        let field_path = quote!(value.field);
        let call_into = generate_field_conversion(&field_path, &None, &None);
        assert_eq!("value . field . into ()", call_into.to_string());
    }

    #[test]
    fn test_with_fn_field_conversion() {
        let field_path = quote!(value.field);
        let with_fn = syn::Path::from_string("fn_name").unwrap();
        let call_into = generate_field_conversion(&field_path, &Some(with_fn), &None);
        assert_eq!("fn_name (value . field)", call_into.to_string());
    }

    #[test]
    fn test_with_fn_ref_field_conversion() {
        let field_path = quote!(value.field);
        let with_ref_fn = syn::Path::from_string("fn_name").unwrap();
        let call_into = generate_field_conversion(&field_path, &None, &Some(with_ref_fn));
        assert_eq!("fn_name (& value . field)", call_into.to_string());
    }

    #[test]
    fn test_apply_field_default() {
        let field_path = quote!(value.field);
        let unwrap_or = syn::Expr::from_string("10").unwrap();
        let call_into = generate_unwrap_or(&field_path, &unwrap_or);
        assert_eq!("value . field . unwrap_or (10)", call_into.to_string());
    }
}
