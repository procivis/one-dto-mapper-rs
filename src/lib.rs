use proc_macro::TokenStream;

use darling::ast::{Data, Fields};
use darling::util::Flag;
use darling::{FromDeriveInput, FromField, FromVariant, ToTokens};
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, ImplGenerics, WhereClause};

mod get_conversion_call;

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

        Ok(self)
    }
}

impl ToTokens for InputReceiver {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let InputReceiver {
            ref ident,
            ref generics,
            ref data,
            ref from,
            ref into,
            ..
        } = self;

        let (ref impl_generics, type_generics, where_clause) = generics.split_for_impl();

        // it's guaranteed from `and_then = "Self::validate_from_or_into"` that there will be at least one of the attributes present
        let from_into_attributes = [
            from.as_ref()
                .map(|from_ident| (quote!(#from_ident), quote!(#ident #type_generics))),
            into.as_ref()
                .map(|into_ident| (quote!(#ident #type_generics), quote!(#into_ident))),
        ];

        for (from_ident, into_ident) in from_into_attributes.iter().filter_map(|o| o.as_ref()) {
            let args = CodegenArgs {
                impl_generics,
                where_clause,
                from_ident,
                into_ident,
            };

            let output = match data {
                Data::Enum(enum_data) => enum_codegen(enum_data, args),
                Data::Struct(struct_data) => named_struct_codegen(struct_data, args),
            };

            tokens.extend(output)
        }
    }
}

#[derive(Debug, FromVariant)]
struct EnumData {
    ident: syn::Ident,
    fields: Fields<FieldsData>,
}

#[derive(Debug, FromField)]
#[darling(
    attributes(convert),
    and_then = "Self::validate_with_fn_and_with_fn_ref"
)]
struct FieldsData {
    ident: Option<syn::Ident>,
    skip: Flag,
    with_fn: Option<syn::Path>,
    with_fn_ref: Option<syn::Path>,
}

impl FieldsData {
    fn validate_with_fn_and_with_fn_ref(self) -> darling::Result<Self> {
        if self.with_fn.is_some() && self.with_fn_ref.is_some() {
            return Err(darling::Error::custom(
                "Only one can be used: (with_fn,with_fn_ref)",
            ));
        }

        Ok(self)
    }
}

struct CodegenArgs<'a> {
    impl_generics: &'a ImplGenerics<'a>,
    where_clause: Option<&'a WhereClause>,
    from_ident: &'a proc_macro2::TokenStream,
    into_ident: &'a proc_macro2::TokenStream,
}

fn named_struct_codegen(
    struct_data: &Fields<FieldsData>,
    CodegenArgs {
        impl_generics,
        where_clause,
        from_ident,
        into_ident,
    }: CodegenArgs,
) -> proc_macro2::TokenStream {
    let fields = &struct_data.fields;

    let value_ident = format_ident!("__value");
    let fields_mapping = fields.iter().filter(|f| !f.skip.is_present()).map(|field| {
        // safe to unwrap since we allow only named structs
        let field_name = field.ident.as_ref().unwrap();

        get_conversion_call::get_conversion_call(
            &value_ident,
            &field_name,
            &field.with_fn,
            &field.with_fn_ref,
        )
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

fn enum_codegen(
    enum_data: &[EnumData],
    CodegenArgs {
        impl_generics,
        where_clause,
        from_ident,
        into_ident,
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
                .filter(|field| !field.skip.is_present())
                .map(|field| {
                    let field_name = field.ident.as_ref().unwrap();

                    quote!(#field_name: #field_name.into())
                });

            quote! {
                #from_ident::#variant_name {
                    #(#named_fields),*
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
