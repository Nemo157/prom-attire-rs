#![recursion_limit = "128"]

#[macro_use]
extern crate error_chain;
extern crate syn;
#[macro_use]
extern crate quote;

mod dissect;
mod expand;
mod errors;

pub use errors::*;

pub struct Config<'a> {
    pub scope: Option<&'a str>,
}

pub fn derive(input: &str, config: Config) -> Result<String> {
    let ast = syn::parse_derive_input(input)?;
    let strukt = dissect::dissect(&ast, &config)?;
    let expanded = expand::expand(&strukt, &config)?;
    Ok(expanded.to_string())
}
