#![recursion_limit = "128"]

#[macro_use]
extern crate error_chain;
extern crate syn;
#[macro_use]
extern crate quote;

mod expand;
mod errors;

pub use errors::*;

pub struct Config<'a> {
    pub scope: Option<&'a str>,
}

pub fn derive(input: &str, config: Config) -> Result<String> {
    let ast = syn::parse_derive_input(input)?;
    Ok(expand::expand(&ast, &config)?.to_string())
}
