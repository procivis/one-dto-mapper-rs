pub(crate) mod enums;
pub(crate) mod fields;

pub(crate) trait Codegen {
    fn generate_conversion(&self) -> Option<proc_macro2::TokenStream>;
    fn generate_conversion_for_ident(&self, ident: &str) -> Option<proc_macro2::TokenStream>;
    fn generate_names(&self) -> Option<proc_macro2::TokenStream>;
}
