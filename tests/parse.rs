#[macro_use]
extern crate hats;
#[macro_use]
extern crate quote;
extern crate syn;

use std::net::{IpAddr, Ipv4Addr};

#[test]
fn parse_string() {
    #[derive(FromAttributes)]
    struct A<'a> {
        b: Option<&'a str>,
    }
    let input = quote! {
        #[b = "test"]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some("test"));
}

#[test]
fn parse_byte_str() {
    #[derive(FromAttributes)]
    struct A<'a> {
        b: Option<&'a [u8]>,
    }
    let input = quote! {
        #[b = "test"]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(&b"test"[..]));
}

#[test]
fn parse_char() {
    #[derive(FromAttributes)]
    struct A {
        b: Option<char>,
    }
    let input = quote! {
        #[b = "b"]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some('b'));
}

#[test]
fn parse_u32() {
    #[derive(FromAttributes)]
    struct A {
        b: Option<u32>,
    }
    let input = quote! {
        #[b = "10"]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(10));
}

#[test]
fn parse_u64() {
    #[derive(FromAttributes)]
    struct A {
        b: Option<u64>,
    }
    let input = quote! {
        #[b = "10"]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(10));
}

#[test]
fn parse_float() {
    #[derive(FromAttributes)]
    struct A {
        b: Option<f64>,
    }
    let input = quote! {
        #[b = "10.01"]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(10.01));
}

#[test]
fn parse_bool() {
    #[derive(FromAttributes)]
    struct A {
        b: Option<bool>,
    }
    let input = quote! {
        #[b = "true"]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(true));
}

#[test]
fn parse_from_str() {
    #[derive(FromAttributes)]
    struct A {
        b: Option<IpAddr>,
    }
    let input = quote! {
        #[b = "127.0.0.1"]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))));
}
