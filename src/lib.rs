#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;

/// "Erase" the code item that follows.
#[proc_macro_attribute]
pub fn erase(attrs: TokenStream, _: TokenStream) -> TokenStream {
    assert!(
        attrs.is_empty(),
        "Attribute nacro flexsync::erase expects no attribute parameters, but it was given: {attrs}"
    );
    TokenStream::new()
}

#[cfg(test)]
mod tests {}
