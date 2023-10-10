#![doc(html_root_url = "https://docs.rs/maud_macros/0.25.0")]
// TokenStream values are reference counted, and the mental overhead of tracking
// lifetimes outweighs the marginal gains from explicit borrowing
#![allow(clippy::needless_pass_by_value)]

extern crate proc_macro;

mod ast;
mod escape;
mod generate;
mod parse;

use proc_macro2::{TokenStream, TokenTree};
use proc_macro_error::proc_macro_error;
use quote::quote;

#[proc_macro]
#[proc_macro_error]
pub fn html_into(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut tokens = TokenStream::from(input).into_iter();
    let output = tokens.next().unwrap();
    let _comma = tokens.next().unwrap();
    let input: TokenStream = tokens.collect();
    expand(output, input).into()
}

fn expand(output_ident: TokenTree, input: TokenStream) -> TokenStream {
    // Heuristic: the size of the resulting markup tends to correlate with the
    // code size of the template itself
    let size_hint = input.to_string().len();
    let markups = parse::parse(input);
    let stmts = generate::generate(markups, output_ident.clone());
    quote!({
        extern crate alloc;
        extern crate maud;
        #output_ident.reserve(#size_hint);
        #stmts
    })
}
