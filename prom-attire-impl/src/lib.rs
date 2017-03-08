#![recursion_limit = "128"]

#[macro_use]
extern crate error_chain;
extern crate syn;
#[macro_use]
extern crate quote;

mod dissect;
mod expand;
mod errors;
mod tmp;

use std::str::FromStr;
pub use errors::*;
use tmp::TryInto;

pub struct Config<'a> {
    pub scope: Option<&'a str>,
    pub docs: Option<&'a str>,
    pub parse_field_config: &'a Fn(&[syn::Attribute]) -> FieldConfig,
}

#[derive(Debug)]
pub struct FieldConfig<'a> {
    pub attribute: Option<&'a str>,
    pub split_attribute_of: Option<&'a str>,
    pub default: Defaulted,
    pub flag_value: Option<&'a str>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Defaulted {
    /// Don't default
    Nope,
    /// Use Default::default as default
    Yep,
    /// Use provided code fragment as default
    To(String),
}

impl Default for FieldConfig<'static> {
    fn default() -> FieldConfig<'static> {
        FieldConfig {
            attribute: None,
            split_attribute_of: None,
            default: Defaulted::Nope,
            flag_value: None,
        }
    }
}

pub fn derive(input: &str, config: Config) -> Result<String> {
    let ast = syn::parse_derive_input(input)?;
    let strukt = (&ast, &config).try_into()?;
    let expanded = expand::expand(&strukt, &config);
    Ok(expanded.to_string())
}

#[doc(hidden)]
pub struct NoError;
impl FromStr for Defaulted {
    // TODO: use !
    type Err = NoError;

    fn from_str(string: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match string {
            "prom_attire_impl::Defaulted::Yep" => Defaulted::Yep,
            "prom_attire_impl::Defaulted::Nope" => Defaulted::Nope,
            _ => Defaulted::To(string.to_owned()),
        })
    }
}
impl ::std::fmt::Debug for NoError {
    fn fmt(&self, _: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        unreachable!()
    }
}
impl ::std::fmt::Display for NoError {
    fn fmt(&self, _: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        unreachable!()
    }
}
impl ::std::error::Error for NoError {
    fn description(&self) -> &str {
        unreachable!()
    }
}
