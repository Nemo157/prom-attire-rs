#[macro_use]
extern crate prom_attire;
#[macro_use]
extern crate quote;
extern crate syn;

use std::net::{IpAddr, Ipv4Addr};

#[test]
fn default_flag_value_bool() {
    #[derive(PromAttire)]
    struct A {
        // bool without a `flag_value` setting should default to true
        b: bool,
    }
    let input = quote! {
        #[b]
        struct S {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, true);
}

#[test]
fn explicit_flag_value_bool() {
    #[derive(PromAttire)]
    struct A {
        #[attire(flag_value = "true")]
        b: bool,
    }
    let input = quote! {
        #[b]
        struct S {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, true);
}

#[test]
fn override_flag_value_bool() {
    #[derive(PromAttire)]
    struct A {
        #[attire(default = "true", flag_value = "false")]
        b: bool,
    }
    let input = quote! {
        #[b]
        struct S {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, false);
}

#[test]
fn flag_value_string_unset() {
    #[derive(PromAttire)]
    struct A {
        #[attire(default = "test", flag_value = "this")]
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
fn flag_value_string_set() {
    #[derive(PromAttire)]
    struct A {
        #[attire(default = "test", flag_value = "this")]
        b: String,
    }
    let input = quote! {
        #[b]
        struct S {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, "this");
}

#[test]
fn flag_value_parsed() {
    #[derive(PromAttire)]
    struct A {
        #[attire(default = "0.0.0.0", flag_value = "127.0.0.1")]
        b: IpAddr,
    }
    let input = quote! {
        #[b]
        struct S {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
}
