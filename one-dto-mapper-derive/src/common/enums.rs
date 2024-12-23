use darling::ast::Fields;
use darling::{FromField, FromVariant};
use proc_macro2::TokenStream;
use quote::quote;

use super::Codegen;

#[derive(Debug, FromVariant)]
pub(crate) struct EnumData<T>
where
    T: FromField,
{
    pub(crate) ident: syn::Ident,
    pub(crate) fields: Fields<T>,
}

impl<T> EnumData<T>
where
    T: FromField + Codegen,
{
    pub(crate) fn generate(&self, from_ident: TokenStream) -> proc_macro2::TokenStream {
        let variant_name = &self.ident;
        let fields = self.fields.as_ref();

        if fields.is_unit() {
            quote!(#from_ident::#variant_name => Self::#variant_name)
        } else if fields.is_newtype() {
            let conversion = fields.fields[0].generate_conversion_for_ident("__val");
            quote!(#from_ident::#variant_name(__val) => Self::#variant_name(#conversion))
        } else if fields.is_struct() {
            let named_fields = fields
                .fields
                .iter()
                .flat_map(|field| field.generate_names());
            let fields_mapping = fields
                .fields
                .iter()
                .flat_map(|field| field.generate_conversion());

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
    }
}
