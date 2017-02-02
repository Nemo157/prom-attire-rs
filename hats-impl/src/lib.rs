#![recursion_limit = "128"]

extern crate syn;
#[macro_use]
extern crate quote;

mod expand;

pub struct Config<'a> {
    pub scope: Option<&'a str>,
}

pub fn derive(input: &str, config: Config) -> String {
    let ast = syn::parse_derive_input(input).unwrap();
    expand::expand(&ast, &config).to_string()
}
