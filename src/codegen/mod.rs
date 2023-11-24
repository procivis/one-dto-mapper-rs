use syn::{ImplGenerics, WhereClause};

pub(crate) mod enums;
pub(crate) mod fields;
pub(crate) mod structs;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum Conversion {
    Into,
    From,
}

pub(crate) struct CodegenArgs<'a> {
    pub(crate) impl_generics: &'a ImplGenerics<'a>,
    pub(crate) where_clause: Option<&'a WhereClause>,
    pub(crate) from_ident: proc_macro2::TokenStream,
    pub(crate) into_ident: proc_macro2::TokenStream,
    pub(crate) conversion: Conversion,
}
