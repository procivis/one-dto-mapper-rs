pub(crate) mod enums;
pub(crate) mod fields;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Conversion {
    Into,
    From,
}

pub(crate) struct Context {
    pub(crate) from_ident: proc_macro2::TokenStream,
    pub(crate) conversion: Conversion,
}

pub(crate) trait Codegen {
    fn generate_conversion(&self, context: &Context) -> Option<proc_macro2::TokenStream>;
    fn generate_conversion_for_ident(
        &self,
        context: &Context,
        ident: &str,
    ) -> Option<proc_macro2::TokenStream>;
    fn generate_names(&self, context: &Context) -> Option<proc_macro2::TokenStream>;
}
