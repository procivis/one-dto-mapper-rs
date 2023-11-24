use quote::{format_ident, quote};

use crate::codegen::Conversion;

use super::fields::{generate_field_conversion, generate_unwrap_or, FieldsData};
use super::CodegenArgs;

pub(crate) fn named_struct_codegen(
    fields: &[FieldsData],
    CodegenArgs {
        impl_generics,
        where_clause,
        from_ident,
        into_ident,
        conversion,
    }: CodegenArgs,
) -> proc_macro2::TokenStream {
    let value_ident = format_ident!("__value");
    let fields_mapping = fields.iter().filter(|f| !f.skip.is_present()).map(|field| {
        // safe to unwrap since we allow only named structs
        let field_name = field.ident.as_ref().unwrap();
        let mut field_path = quote!(#value_ident.#field_name);

        let unwrap_or = field.unwrap_or.as_ref();

        field_path = match conversion {
            Conversion::Into => unwrap_or.or(field.into_unwrap_or.as_ref()),
            Conversion::From => unwrap_or.or(field.from_unwrap_or.as_ref()),
        }
        .map(|unwrap_or| generate_unwrap_or(&field_path, unwrap_or))
        .unwrap_or(field_path);

        let conversion = generate_field_conversion(&field_path, &field.with_fn, &field.with_fn_ref);

        quote!(#field_name: #conversion)
    });

    quote! {
        impl #impl_generics From<#from_ident> for #into_ident #where_clause {
            fn from(#value_ident: #from_ident) -> Self {
                Self {
                    #(#fields_mapping),*
                }
            }
        }
    }
}
