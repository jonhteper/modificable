use proc_macro::TokenStream;

mod modificable_derive;

/// Implement setters for structs with a field that implements `[Modifications]` trait.
#[proc_macro_derive(Modificable, attributes(modifications, setter))]
pub fn derive_modificable(input: TokenStream) -> TokenStream {
    modificable_derive::modificable_macro(input.into()).into()
}
