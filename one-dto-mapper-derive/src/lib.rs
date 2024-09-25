use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod common;
mod from;
mod into;
mod try_from;
mod try_into;

// TODO:
// - add examples
// - improve tests

#[proc_macro_derive(From, attributes(from))]
pub fn derive_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = match from::FromInputReceiver::from_derive_input(&input) {
        Err(error) => return TokenStream::from(error.write_errors()),
        Ok(out) => out,
    };

    quote!(#output).into()
}

#[proc_macro_derive(Into, attributes(into))]
pub fn derive_into(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = match into::IntoInputReceiver::from_derive_input(&input) {
        Err(error) => return TokenStream::from(error.write_errors()),
        Ok(out) => out,
    };

    quote!(#output).into()
}

#[proc_macro_derive(TryFrom, attributes(try_from))]
pub fn derive_try_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = match try_from::TryFromInputReceiver::from_derive_input(&input) {
        Err(error) => return TokenStream::from(error.write_errors()),
        Ok(out) => out,
    };

    quote!(#output).into()
}

#[proc_macro_derive(TryInto, attributes(try_into))]
pub fn derive_try_into(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = match try_into::TryIntoInputReceiver::from_derive_input(&input) {
        Err(error) => return TokenStream::from(error.write_errors()),
        Ok(out) => out,
    };

    quote!(#output).into()
}
