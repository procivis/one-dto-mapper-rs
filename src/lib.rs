use codegen::enums::{enum_codegen, EnumData};
use codegen::fields::FieldsData;
use codegen::structs::named_struct_codegen;
use codegen::{CodegenArgs, Conversion};
use darling::ast::Data;
use darling::{FromDeriveInput, ToTokens};
use itertools::Either;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod codegen;

// TODO:
// - add examples
// - improve tests
// - add renaming when mapping (should be quite easy)

/// Derives `From` for you assuming that the shape of the type you're converting from/into is the same.
#[proc_macro_derive(From, attributes(convert))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = match InputReceiver::from_derive_input(&input) {
        Err(error) => return TokenStream::from(error.write_errors()),
        Ok(out) => out,
    };

    quote!(#output).into()
}

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(convert),
    supports(struct_named, enum_unit, enum_named, enum_newtype),
    and_then = "Self::validate_from_or_into"
)]
struct InputReceiver {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<EnumData, FieldsData>,
    /// Attribute #[convert(into = "...")]
    into: Option<syn::Path>,
    /// Attribute #[convert(from = "...")]
    from: Option<syn::Path>,
}

impl InputReceiver {
    fn validate_from_or_into(self) -> darling::Result<Self> {
        if self.from.is_none() && self.into.is_none() {
            return Err(darling::Error::custom("Missing one of the attributes '#[convert(from = \"...\")]' or '#[convert(into = \"...\")]'"));
        }

        let directions: Vec<_> = [
            self.from.as_ref().map(|_| Conversion::From),
            self.into.as_ref().map(|_| Conversion::Into),
        ]
        .into_iter()
        .flatten()
        .collect();

        match &self.data {
            Data::Enum(fields) => Either::Left(fields.iter().flat_map(|entry| entry.fields.iter())),
            Data::Struct(fields) => Either::Right(fields.fields.iter()),
        }
        .try_for_each(|field| field.validate_with_directions(&directions))?;

        Ok(self)
    }
}

impl ToTokens for InputReceiver {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let InputReceiver {
            ident,
            generics,
            data,
            from,
            into,
        } = self;

        let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

        // it's guaranteed from `and_then = "Self::validate_from_or_into"` that there will be at least one of the attributes present
        let from_into_attributes = [
            from.as_ref().map(|from_ident| {
                (
                    quote!(#from_ident),
                    quote!(#ident #type_generics),
                    Conversion::From,
                )
            }),
            into.as_ref().map(|into_ident| {
                (
                    quote!(#ident #type_generics),
                    quote!(#into_ident),
                    Conversion::Into,
                )
            }),
        ];

        for (from_ident, into_ident, conversion) in from_into_attributes.into_iter().flatten() {
            let args = CodegenArgs {
                impl_generics: &impl_generics,
                where_clause,
                from_ident,
                into_ident,
                conversion,
            };

            let output = match data {
                Data::Enum(enum_data) => enum_codegen(enum_data, args),
                Data::Struct(struct_data) => named_struct_codegen(&struct_data.fields, args),
            };

            tokens.extend(output)
        }
    }
}
