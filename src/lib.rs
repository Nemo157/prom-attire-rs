//! <center>
//!   <b>Pro</b>cedural <b>M</b>acro <b>Attri</b>but<b>e</b>s
//!   <br>
//!   For when you need the best dressed procedural macro.
//! </center>
//!
//! `prom_attire` lets you define a struct (or multiple) that you can use to
//! parse the attributes passed in to your procedural macro.

//! # Examples

//! ## Basic example
//!
//! The simplest example is taking in a string value for an attribute. Unless
//! you use some of the customization shown later, you need to wrap all
//! attributes in `Option` in case the user does not specify them.
//!
//! ```rust
//! # #[macro_use] extern crate prom_attire;
//! # extern crate syn;
//! # fn main() {
//! # struct E;
//! # impl ::std::fmt::Debug for E { fn fmt(&self, _: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { Ok(()) } }
//! # impl From<String> for E { fn from(_: String) -> E { E } }
//! # impl<T> From<Vec<T>> for E { fn from(_: Vec<T>) -> E { E } }
//! # fn foo() -> Result<(), E> {
//! #[derive(PromAttire, PartialEq, Debug)]
//! struct Attributes {
//!     awesome: Option<String>,
//! }
// TODO: rustdoc has issues with raw string literals, switch to those once
// fixed (extra TODO, open rust-lang/rust bug ticket about them and link)
//! let ast = syn::parse_derive_input("
//!     #[awesome = \"yes\"]
//!     struct Foo {}
//! ")?;
//! let attrs = Attributes::try_from(ast.attrs.as_slice())?;
//! assert_eq!(attrs, Attributes {
//!     awesome: Some("yes".to_owned()),
//! });
//! # Ok(())
//! # }
//! # foo().unwrap()
//! # }
//! ```

//! ## Parsing
//!
//! `prom_attire` is able to parse to any type that implements
//! [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html).
//!
//! ```rust
//! # #[macro_use] extern crate prom_attire;
//! # extern crate syn;
//! # fn main() {
//! # struct E;
//! # impl ::std::fmt::Debug for E { fn fmt(&self, _: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { Ok(()) } }
//! # impl From<String> for E { fn from(_: String) -> E { E } }
//! # impl<T> From<Vec<T>> for E { fn from(_: Vec<T>) -> E { E } }
//! # fn foo() -> Result<(), E> {
//! use std::net::{IpAddr, Ipv4Addr};
//! #[derive(PromAttire, PartialEq, Debug)]
//! struct Attributes {
//!     rust_lang: Option<IpAddr>,
//! }
//! let ast = syn::parse_derive_input("
//!     #[rust_lang = \"31.220.0.199\"]
//!     struct Foo {}
//! ")?;
//! let attrs = Attributes::try_from(ast.attrs.as_slice())?;
//! assert_eq!(attrs, Attributes {
//!     rust_lang: Some(IpAddr::V4(Ipv4Addr::new(31, 220, 0, 199))),
//! });
//! # Ok(())
//! # }
//! # foo().unwrap()
//! # }
//! ```

//! ## Errors
//!
//! There are two methods added to your attribute type, one is an
//! implementation of `From<&[syn::Attribute]>` that will panic if any error
//! occurs. The other has the signature `fn try_from(attrs: &[syn::Attribute])
//! -> Result<A, Vec<E>>` where `A` is your attribute type and `E` is a
//! generated error type named `YourStructName + FromAttributesError`.
//!
//! The most common error will be that parsing the value provided by your users
//! failed, inspecting that error could look something like
//!
//! ```rust
//! # #[macro_use] extern crate prom_attire;
//! # extern crate syn;
//! # fn main() {
//! # struct E;
//! # impl ::std::fmt::Debug for E { fn fmt(&self, _: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { Ok(()) } }
//! # impl From<String> for E { fn from(_: String) -> E { E } }
//! # impl<T> From<Vec<T>> for E { fn from(_: Vec<T>) -> E { E } }
//! # fn foo() -> Result<(), E> {
//! use std::net::{IpAddr, Ipv4Addr};
//! #[derive(PromAttire, PartialEq, Debug)]
//! struct Attributes {
//!     boom: Option<IpAddr>
//! }
//! let ast = syn::parse_derive_input("
//!     #[boom = \"31.220.0\"]
//!     struct Foo {}
//! ")?;
//! let errs = Attributes::try_from(ast.attrs.as_slice()).unwrap_err();
//! if let AttributesFromAttributesError::Parsing { value, attr, ref err, .. } = errs[0] {
//!     assert_eq!(value, "31.220.0");
//!     assert_eq!(attr, "boom");
//!     // assert_eq!(err, Box::new("31.220.0".parse::<IpAddr>().unwrap_err()));
//! }
//! # Ok(())
//! # }
//! # foo().unwrap()
//! # }
//! ```
//!
//! These errors should have decent error messages in their `Display`
//! implementation, it is recommended that you treat them as opaque errors as
//! much as possible and open bug tickets if there are some enhancements that
//! you feel would make the resulting messages nicer for your users. However
//! they are (at least currently) exposed to the module in which your attribute
//! struct is defined if you need to pull details from them.

//! ## Lifetimes
//!
//! So far these examples have shown only examples of owned types. You can also
//! use types that borrow from the provided AST by specifying a single lifetime
//! on the struct. (Currently this only supports `&str` as `FromStr` does not
//! support borrowing from the input, this may be extended in the future).
//!
//! ```rust
//! # #[macro_use] extern crate prom_attire;
//! # extern crate syn;
//! # fn main() {
//! # struct E;
//! # impl ::std::fmt::Debug for E { fn fmt(&self, _: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { Ok(()) } }
//! # impl From<String> for E { fn from(_: String) -> E { E } }
//! # impl<T> From<Vec<T>> for E { fn from(_: Vec<T>) -> E { E } }
//! # fn foo() -> Result<(), E> {
//! #[derive(PromAttire, PartialEq, Debug)]
//! struct Attributes<'a> {
//!     awesome: Option<&'a str>,
//! }
//! let ast = syn::parse_derive_input("
//!     #[awesome = \"yes\"]
//!     struct Foo {}
//! ")?;
//! let attrs = Attributes::try_from(ast.attrs.as_slice())?;
//! assert_eq!(attrs, Attributes {
//!     awesome: Some("yes"),
//! });
//! # Ok(())
//! # }
//! # foo().unwrap()
//! # }
//! ```

//! ## Multiple Values
//!
//! If you wish to take in multiple values for an attribute, just wrap the
//! attribute type in `Vec` instead of `Option`. Every instance of the
//! attribute will be appended to the vector.
//!
//! ```rust
//! # #[macro_use] extern crate prom_attire;
//! # extern crate syn;
//! # fn main() {
//! # struct E;
//! # impl ::std::fmt::Debug for E { fn fmt(&self, _: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { Ok(()) } }
//! # impl From<String> for E { fn from(_: String) -> E { E } }
//! # impl<T> From<Vec<T>> for E { fn from(_: Vec<T>) -> E { E } }
//! # fn foo() -> Result<(), E> {
//! #[derive(PromAttire, PartialEq, Debug)]
//! struct Attributes<'a> {
//!     awesome: Vec<&'a str>,
//! }
//! let ast = syn::parse_derive_input("
//!     #[awesome = \"yes\"]
//!     #[awesome = \"always\"]
//!     struct Foo {}
//! ")?;
//! let attrs = Attributes::try_from(ast.attrs.as_slice())?;
//! assert_eq!(attrs, Attributes {
//!     awesome: vec!["yes", "always"],
//! });
//! # Ok(())
//! # }
//! # foo().unwrap()
//! # }
//! ```

//! ## Scoping
//!
//! Commonly you will want to namespace your attributes, this is used by
//! `prom-attire` itself to keep all the configuration under a top-level
//! `attire` attribute. To set the namespace just add an `#[attire(scope =
//! "foo")]` attribute to the struct.
//!
//! ```rust
//! # #[macro_use] extern crate prom_attire;
//! # extern crate syn;
//! # fn main() {
//! # struct E;
//! # impl ::std::fmt::Debug for E { fn fmt(&self, _: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { Ok(()) } }
//! # impl From<String> for E { fn from(_: String) -> E { E } }
//! # impl<T> From<Vec<T>> for E { fn from(_: Vec<T>) -> E { E } }
//! # fn foo() -> Result<(), E> {
//! #[derive(PromAttire, PartialEq, Debug)]
//! #[attire(scope = "you")]
//! struct Attributes<'a> {
//!     awesome: Option<&'a str>,
//! }
//! let ast = syn::parse_derive_input("
//!     #[you(awesome = \"yes\")]
//!     struct Foo {}
//! ")?;
//! let attrs = Attributes::try_from(ast.attrs.as_slice())?;
//! assert_eq!(attrs, Attributes {
//!     awesome: Some("yes"),
//! });
//! # Ok(())
//! # }
//! # foo().unwrap()
//! # }
//! ```

//! ## Default
//!
//! Instead of having to check whether each attribute is `Some` useful value or
//! using a default if it is `None` manually, you can tell `prom-attire` to
//! inject a default value if not specified with an `#[attire(default =
//! "foo")]` attribute on the field.
//!
//! ```rust
//! # #[macro_use] extern crate prom_attire;
//! # extern crate syn;
//! # fn main() {
//! # struct E;
//! # impl ::std::fmt::Debug for E { fn fmt(&self, _: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { Ok(()) } }
//! # impl From<String> for E { fn from(_: String) -> E { E } }
//! # impl<T> From<Vec<T>> for E { fn from(_: Vec<T>) -> E { E } }
//! # fn foo() -> Result<(), E> {
//! #[derive(PromAttire, PartialEq, Debug)]
//! struct Attributes {
//!     #[attire(default = "yes")]
//!     awesome: String,
//!     #[attire(default = "no")]
//!     overridable: String,
//! }
//! let ast = syn::parse_derive_input("
//!     #[overridable = \"yes!\"]
//!     struct Foo {}
//! ")?;
//! let attrs = Attributes::try_from(ast.attrs.as_slice())?;
//! assert_eq!(attrs, Attributes {
//!     awesome: "yes".to_owned(),
//!     overridable: "yes!".to_owned(),
//! });
//! # Ok(())
//! # }
//! # foo().unwrap()
//! # }
//! ```

//! ## More examples **Coming Soon**
//!
//! For now if you check [the list of examples to
//! add](https://github.com/Nemo157/prom-attire-rs/issues/5) and [look at the
//! tests](https://github.com/Nemo157/prom-attire-rs/tree/master/tests) you may
//! be able to work out how to do what you want.


#[macro_use]
extern crate prom_attire_derive;

pub use prom_attire_derive::*;
