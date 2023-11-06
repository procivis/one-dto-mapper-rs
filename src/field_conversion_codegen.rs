use quote::quote;

pub(super) fn field_conversion_codegen(
    value_ident: &syn::Ident,
    field_name: &syn::Ident,
    with_fn: &Option<syn::Path>,
    with_fn_ref: &Option<syn::Path>,
) -> proc_macro2::TokenStream {
    if let Some(function) = with_fn {
        return quote!(#field_name: #function(#value_ident.#field_name));
    }

    if let Some(function) = with_fn_ref {
        return quote!(#field_name: #function(&#value_ident.#field_name));
    }

    quote!(#field_name: #value_ident.#field_name.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use darling::FromMeta;

    #[test]
    fn test_field_conversion_codegen() {
        let value_ident = syn::Ident::from_string("value").unwrap();
        let field_name = syn::Ident::from_string("field").unwrap();

        let fn_name = syn::Path::from_string("fn_name").unwrap();

        let call_into = field_conversion_codegen(&value_ident, &field_name, &None, &None);
        assert_eq!("field : value . field . into ()", call_into.to_string());

        let call_with_fn =
            field_conversion_codegen(&value_ident, &field_name, &Some(fn_name.to_owned()), &None);
        assert_eq!("field : fn_name (value . field)", call_with_fn.to_string());

        let call_with_fn_ref =
            field_conversion_codegen(&value_ident, &field_name, &None, &Some(fn_name.to_owned()));
        assert_eq!(
            "field : fn_name (& value . field)",
            call_with_fn_ref.to_string()
        );
    }
}
