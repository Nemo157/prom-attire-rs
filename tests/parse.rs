#[macro_use]
extern crate prom_attire;
#[macro_use]
extern crate quote;
extern crate syn;

use std::net::{IpAddr, Ipv4Addr};

#[test]
fn parse_string() {
    #[derive(PromAttire)]
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
    #[derive(PromAttire)]
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
    #[derive(PromAttire)]
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
fn parse_u8() {
    #[derive(PromAttire)]
    struct A {
        b: Option<u8>,
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
fn parse_i8() {
    #[derive(PromAttire)]
    struct A {
        b: Option<i8>,
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
fn parse_u16() {
    #[derive(PromAttire)]
    struct A {
        b: Option<u16>,
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
fn parse_i16() {
    #[derive(PromAttire)]
    struct A {
        b: Option<i16>,
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
fn parse_u32() {
    #[derive(PromAttire)]
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
fn parse_i32() {
    #[derive(PromAttire)]
    struct A {
        b: Option<i32>,
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
    #[derive(PromAttire)]
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
fn parse_i64() {
    #[derive(PromAttire)]
    struct A {
        b: Option<i64>,
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
fn parse_usize() {
    #[derive(PromAttire)]
    struct A {
        b: Option<usize>,
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
fn parse_isize() {
    #[derive(PromAttire)]
    struct A {
        b: Option<isize>,
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
fn parse_f32() {
    #[derive(PromAttire)]
    struct A {
        b: Option<f32>,
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
fn parse_f64() {
    #[derive(PromAttire)]
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
    #[derive(PromAttire)]
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
    #[derive(PromAttire)]
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

#[test]
#[cfg(never_type)]
fn parse_from_infallible() {
    use std::str::FromStr;
    #[derive(Debug, Eq, PartialEq)]
    struct Infallible(String);
    impl FromStr for Infallible {
        type Err = !;
        fn from_str(s: &str) -> Result<Infallible, !> {
            Ok(Infallible(s.to_owned()))
        }
    }
    #[derive(PromAttire)]
    struct A {
        b: Option<Infallible>,
    }
    let input = quote! {
        #[b = "boom"]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(Infallible("boom".to_owned())));
}
