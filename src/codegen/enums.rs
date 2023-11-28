use darling::ast::Fields;
use darling::FromVariant;
use quote::quote;

use crate::codegen::Conversion;

use super::fields::{generate_field_conversion, generate_unwrap_or, FieldsData};
use super::CodegenArgs;

#[derive(Debug, FromVariant)]
pub(crate) struct EnumData {
    pub(crate) ident: syn::Ident,
    pub(crate) fields: Fields<FieldsData>,
}

pub(crate) fn enum_codegen(
    enum_data: &[EnumData],
    CodegenArgs {
        impl_generics,
        where_clause,
        from_ident,
        into_ident,
        conversion,
    }: CodegenArgs,
) -> proc_macro2::TokenStream {
    let body = enum_data.iter().map(|variant| {
        let variant_name = &variant.ident;
        let fields = variant.fields.as_ref();

        if fields.is_unit() {
            quote!(#from_ident::#variant_name => Self::#variant_name)
        } else if fields.is_newtype() {
            quote!(#from_ident::#variant_name(__val) => Self::#variant_name(__val.into()))
        } else if fields.is_struct() {
            let named_fields = fields
                .fields
                .iter()
                .filter(|field| !field.skip.is_present())
                .filter_map(|field| field.ident.as_ref());

            let fields_mapping = fields
                .fields
                .iter()
                .filter(|f| match conversion {
                    Conversion::Into => !f.skip.is_present(),
                    Conversion::From => true,
                })
                .map(|field| {
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

                    let mut field_path = quote!(#right_field_name);

                    let unwrap_or = field.unwrap_or.as_ref();

                    field_path = match conversion {
                        Conversion::Into => unwrap_or.or(field.into_unwrap_or.as_ref()),
                        Conversion::From => unwrap_or.or(field.from_unwrap_or.as_ref()),
                    }
                    .map(|unwrap_or| generate_unwrap_or(&field_path, unwrap_or))
                    .unwrap_or(field_path);

                    let conversion = match &field.replace {
                        Some(expr) => quote!(#expr.into()),
                        None => generate_field_conversion(
                            &field_path,
                            &field.with_fn,
                            &field.with_fn_ref,
                        ),
                    };
                    quote!(#left_field_name: #conversion)
                });

            quote! {
                #from_ident::#variant_name {
                    #(#named_fields),*
                    ,..
                } => Self::#variant_name {
                    #(#fields_mapping),*
                }
            }
        } else {
            darling::Error::unsupported_shape("tuple enum").write_errors()
        }
    });

    quote! {
        impl #impl_generics From<#from_ident> for #into_ident #where_clause {
            fn from(__value: #from_ident) -> Self {
                match __value {
                    #(#body),*
                }
            }
        }
    }
}
