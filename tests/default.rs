#[macro_use]
extern crate prom_attire;
#[macro_use]
extern crate quote;
extern crate syn;

use std::net::{IpAddr, Ipv4Addr};

#[test]
fn default_bool() {
    #[derive(PromAttire)]
    struct A {
        // bool without a `default` setting should default to defaulting
        b: bool,
    }
    let input = quote! {
        struct S {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, false);
}

#[test]
fn explicit_default_bool() {
    #[derive(PromAttire)]
    struct A {
        #[attire(default)]
        b: bool,
    }
    let input = quote! {
        struct S {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, false);
}

#[test]
fn override_default_bool() {
    #[derive(PromAttire)]
    struct A {
        #[attire(default = "true")]
        b: bool,
    }
    let input = quote! {
        struct S {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, true);
}

#[test]
fn override_default_bool_literal() {
    #[derive(PromAttire)]
    struct A {
        #[attire(default = "true")]
        b: bool,
    }
    let input = quote! {
        struct S {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, true);
}

#[test]
fn default_string() {
    #[derive(PromAttire)]
    struct A {
        #[attire(default)]
        b: String,
    }
    let input = quote! {
        struct S {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, "");
}

#[test]
fn specified_default_string() {
    #[derive(PromAttire)]
    struct A {
        #[attire(default = "test")]
        b: String,
    }
    let input = quote! {
        struct S {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, "test");
}

#[test]
fn default_string_with_value() {
    #[derive(PromAttire)]
    struct A {
        #[attire(default)]
        b: String,
    }
    let input = quote! {
        #[b = "this"]
        struct S {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, "this");
}

#[test]
fn specified_default_string_with_value() {
    #[derive(PromAttire)]
    struct A {
        #[attire(default = "test")]
        b: String,
    }
    let input = quote! {
        #[b = "this"]
        struct S {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, "this");
}

#[test]
fn default_parsed() {
    #[derive(PromAttire)]
    struct A {
        #[attire(default = "127.0.0.1")]
        b: IpAddr,
    }
    let input = quote! {
        struct S {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
}
