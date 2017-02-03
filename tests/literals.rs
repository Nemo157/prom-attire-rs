#[macro_use]
extern crate prom_attire;
#[macro_use]
extern crate quote;
extern crate syn;

#[test]
fn literal_string() {
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
fn literal_byte_str() {
    #[derive(FromAttributes)]
    struct A<'a> {
        b: Option<&'a [u8]>,
    }
    let input = quote! {
        #[b = b"test"]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(&b"test"[..]));
}

#[test]
fn literal_char() {
    #[derive(FromAttributes)]
    struct A {
        b: Option<char>,
    }
    let input = quote! {
        #[b = 'b']
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some('b'));
}

#[test]
fn literal_u8() {
    #[derive(FromAttributes)]
    struct A {
        b: Option<u8>,
    }
    let input = quote! {
        #[b = 10]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(10));
}

#[test]
fn literal_i8() {
    #[derive(FromAttributes)]
    struct A {
        b: Option<i8>,
    }
    let input = quote! {
        #[b = 10]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(10));
}

#[test]
fn literal_u16() {
    #[derive(FromAttributes)]
    struct A {
        b: Option<u16>,
    }
    let input = quote! {
        #[b = 10]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(10));
}

#[test]
fn literal_i16() {
    #[derive(FromAttributes)]
    struct A {
        b: Option<i16>,
    }
    let input = quote! {
        #[b = 10]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(10));
}

#[test]
fn literal_u32() {
    #[derive(FromAttributes)]
    struct A {
        b: Option<u32>,
    }
    let input = quote! {
        #[b = 10]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(10));
}

#[test]
fn literal_i32() {
    #[derive(FromAttributes)]
    struct A {
        b: Option<i32>,
    }
    let input = quote! {
        #[b = 10]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(10));
}

#[test]
fn literal_u64() {
    #[derive(FromAttributes)]
    struct A {
        b: Option<u64>,
    }
    let input = quote! {
        #[b = 10]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(10));
}

#[test]
fn literal_i64() {
    #[derive(FromAttributes)]
    struct A {
        b: Option<i64>,
    }
    let input = quote! {
        #[b = 10]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(10));
}

#[test]
fn literal_usize() {
    #[derive(FromAttributes)]
    struct A {
        b: Option<usize>,
    }
    let input = quote! {
        #[b = 10]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(10));
}

#[test]
fn literal_isize() {
    #[derive(FromAttributes)]
    struct A {
        b: Option<isize>,
    }
    let input = quote! {
        #[b = 10]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(10));
}

#[test]
fn literal_f32() {
    #[derive(FromAttributes)]
    struct A {
        b: Option<f32>,
    }
    let input = quote! {
        #[b = 10.01]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(10.01));
}

#[test]
fn literal_f64() {
    #[derive(FromAttributes)]
    struct A {
        b: Option<f64>,
    }
    let input = quote! {
        #[b = 10.01]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(10.01));
}

#[test]
fn literal_bool() {
    #[derive(FromAttributes)]
    struct A {
        b: Option<bool>,
    }
    let input = quote! {
        #[b = true]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(true));
}
