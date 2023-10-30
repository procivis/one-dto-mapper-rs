use quote::quote;

pub(super) fn get_conversion_call(
    value_ident: &syn::Ident,
    field_name: &syn::Ident,
    with_fn: &Option<syn::Path>,
    with_fn_ref: &Option<syn::Path>,
) -> proc_macro2::TokenStream {
    let result = get_function_and_operator(with_fn, with_fn_ref);

    match result {
        Ok(None) => quote!(#field_name: #value_ident.#field_name.into()),
        Ok(Some(value)) => {
            let FunctionAndOperator {
                function, operator, ..
            } = value;
            quote!(#field_name: #function(#operator #value_ident.#field_name))
        }
        Err(error) => error.write_errors(),
    }
}

struct FunctionAndOperator {
    pub function: syn::Path,
    pub operator: Option<proc_macro2::TokenStream>,
}

fn get_function_and_operator(
    with_fn: &Option<syn::Path>,
    with_fn_ref: &Option<syn::Path>,
) -> Result<Option<FunctionAndOperator>, darling::Error> {
    if with_fn.is_none() && with_fn_ref.is_none() {
        Ok(None)
    } else if let Some(with_fn) = with_fn {
        Ok(Some(FunctionAndOperator {
            function: with_fn.to_owned(),
            operator: None,
        }))
    } else if let Some(with_fn_ref) = with_fn_ref {
        Ok(Some(FunctionAndOperator {
            function: with_fn_ref.to_owned(),
            operator: Some("&".parse().unwrap()),
        }))
    } else {
        Err(darling::Error::custom(
            "get_function_and_operator(): Unreachable code",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use darling::FromMeta;

    #[test]
    fn test_get_conversion_call() {
        let value_ident = syn::Ident::from_string("value").unwrap();
        let field_name = syn::Ident::from_string("field").unwrap();

        let fn_name = syn::Path::from_string("fn_name").unwrap();

        let call_into = get_conversion_call(&value_ident, &field_name, &None, &None);
        assert_eq!("field : value . field . into ()", call_into.to_string());

        let call_with_fn =
            get_conversion_call(&value_ident, &field_name, &Some(fn_name.to_owned()), &None);
        assert_eq!("field : fn_name (value . field)", call_with_fn.to_string());

        let call_with_fn_ref =
            get_conversion_call(&value_ident, &field_name, &None, &Some(fn_name.to_owned()));
        assert_eq!(
            "field : fn_name (& value . field)",
            call_with_fn_ref.to_string()
        );
    }
}
