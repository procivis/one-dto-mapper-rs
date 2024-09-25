use quote::quote;

pub(crate) fn generate_unwrap_or(
    field_path: &proc_macro2::TokenStream,
    unwrap_or: &syn::Expr,
) -> proc_macro2::TokenStream {
    quote!(#field_path.unwrap_or(#unwrap_or))
}

pub(crate) fn generate_field_conversion(
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

pub(crate) fn generate_field_try_conversion(
    field_path: &proc_macro2::TokenStream,
    with_fn: &Option<syn::Path>,
    with_fn_ref: &Option<syn::Path>,
) -> proc_macro2::TokenStream {
    if let Some(function) = with_fn {
        return quote!(#function(#field_path)?);
    }

    if let Some(function) = with_fn_ref {
        return quote!(#function(&#field_path)?);
    }

    quote!(#field_path.try_into()?)
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
    fn test_into_field_try_conversion() {
        let field_path = quote!(value.field);
        let call_into = generate_field_try_conversion(&field_path, &None, &None);
        assert_eq!("value . field . try_into () ?", call_into.to_string());
    }

    #[test]
    fn test_with_fn_field_try_conversion() {
        let field_path = quote!(value.field);
        let with_fn = syn::Path::from_string("fn_name").unwrap();
        let call_into = generate_field_try_conversion(&field_path, &Some(with_fn), &None);
        assert_eq!("fn_name (value . field) ?", call_into.to_string());
    }

    #[test]
    fn test_with_fn_ref_field_try_conversion() {
        let field_path = quote!(value.field);
        let with_ref_fn = syn::Path::from_string("fn_name").unwrap();
        let call_into = generate_field_try_conversion(&field_path, &None, &Some(with_ref_fn));
        assert_eq!("fn_name (& value . field) ?", call_into.to_string());
    }

    #[test]
    fn test_apply_field_default() {
        let field_path = quote!(value.field);
        let unwrap_or = syn::Expr::from_string("10").unwrap();
        let call_into = generate_unwrap_or(&field_path, &unwrap_or);
        assert_eq!("value . field . unwrap_or (10)", call_into.to_string());
    }
}
