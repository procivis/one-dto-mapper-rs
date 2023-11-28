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
    let fields_mapping = fields
        .iter()
        .filter(|f| match conversion {
            Conversion::Into => !f.skip.is_present(),
            Conversion::From => true,
        })
        .map(|field| {
            // safe to unwrap since we allow only named structs
            let left_field_name = match conversion {
                Conversion::Into => field.rename.as_ref().or(field.ident.as_ref()),
                Conversion::From => field.ident.as_ref(),
            }
            .unwrap();

            let right_field_name = match conversion {
                Conversion::Into => field.ident.as_ref(),
                Conversion::From => field.rename.as_ref().or(field.ident.as_ref()),
            }
            .unwrap();

            let mut field_path = quote!(#value_ident.#right_field_name);

            let unwrap_or = field.unwrap_or.as_ref();

            field_path = match conversion {
                Conversion::Into => unwrap_or.or(field.into_unwrap_or.as_ref()),
                Conversion::From => unwrap_or.or(field.from_unwrap_or.as_ref()),
            }
            .map(|unwrap_or| generate_unwrap_or(&field_path, unwrap_or))
            .unwrap_or(field_path);

            let conversion = match &field.replace {
                Some(expr) => quote!(#expr.into()),
                None => generate_field_conversion(&field_path, &field.with_fn, &field.with_fn_ref),
            };
            quote!(#left_field_name: #conversion)
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
