//! <center>
//!   <b>Pro</b>cedural <b>M</b>acro <b>Attri</b>but<b>e</b>s
//!   <br>
//!   For when you need the best dressed procedural macro.
//! </center>
//!
//! `prom_attire` lets you define a struct (or multiple) that you can use to
//! parse the attributes passed in to your procedural macro.
//!
//! # Examples
//!
//! ## Basic example
//!
// TODO: For some reason this isn't being picked up as a doctest...
//! ```rust
//! #[derive(PromAttire, PartialEq, Debug)]
//! struct Attributes {
//!     awesome: Option<&'a str>,
//! }
//! let ast = syn::parse_derive_input("
//!     #[awesome = \"yes\"]
//!     struct Foo {}
//! ");
//! let attrs = Attributes::from(ast.attrs.as_slice());
//! assert_eq!(attrs, Attributes {
//!     awesome: Some("yes"),
//! })
//! ```

extern crate proc_macro;
extern crate syn;
extern crate error_chain;

#[macro_use]
extern crate prom_attire_bootstrap;
extern crate prom_attire_impl;

use error_chain::ChainedError;

#[derive(PromAttireBootstrap)]
struct Attributes<'a> {
    #[attire_bootstrap(scope)]
    scope: Option<&'a str>,
    #[attire_bootstrap(docs)]
    docs: Option<&'a str>,
}

#[derive(PromAttireBootstrap)]
struct FieldAttributes<'a> {
    #[attire_bootstrap(field_attribute)]
    attribute: Option<&'a str>,
    #[attire_bootstrap(field_split_attribute_of)]
    split_attribute_of: Option<&'a str>,
    #[attire_bootstrap(field_default)]
    default: prom_attire_impl::Defaulted,
    #[attire_bootstrap(field_flag_value)]
    flag_value: Option<&'a str>,
}

/// The procedural macro implementing `#[derive(PromAttire)]`
#[proc_macro_derive(PromAttire, attributes(attire))]
pub fn app(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = input.to_string();

    let ast = match syn::parse_derive_input(&input) {
        Ok(ast) => ast,
        Err(err) => {
            println!("{}", err);
            panic!("Internal error in prom-attire (probably)");
        }
    };

    let attrs = match Attributes::try_from(ast.attrs.as_slice()) {
        Ok(attrs) => attrs,
        Err(errs) => {
            for err in errs {
                println!("{}", err);
            }
            panic!("Invalid attributes specified for prom-attire macro");
        }
    };

    let config = prom_attire_impl::Config {
        scope: attrs.scope,
        docs: attrs.docs,
        parse_field_config: &|attrs| {
            let attrs = match FieldAttributes::try_from(attrs) {
                Ok(attrs) => attrs,
                Err(errs) => {
                    for err in errs {
                        println!("{}", err);
                    }
                    panic!("Invalid attributes specified for prom-attire macro");
                }
            };
            prom_attire_impl::FieldConfig {
                attribute: attrs.attribute,
                split_attribute_of: attrs.split_attribute_of,
                default: attrs.default,
                flag_value: attrs.flag_value,
            }
        }
    };

    let expanded = match prom_attire_impl::derive(&input, &config) {
        Ok(expanded) => expanded,
        Err(err) => {
            println!("{}", err.display());
            panic!("Expanding prom-attire failed");
        }
    };

    match expanded.parse() {
        Ok(parsed) => parsed,
        Err(err) => {
            println!("{:?}", err);
            panic!("Internal error in prom-attire");
        }
    }
}
