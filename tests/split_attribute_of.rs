#[macro_use]
extern crate prom_attire;
#[macro_use]
extern crate quote;
extern crate syn;

#[test]
fn split_attribute_of_parent() {
    #[derive(PromAttire)]
    struct A<'a> {
        #[attire(split_attribute_of = "b")]
        c: Option<&'a str>,
        #[attire(split_attribute_of = "b")]
        d: Option<&'a str>,
    }
    let input = quote! {
        #[b = "b"]
        struct S {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.c, Some("b"));
    assert_eq!(attrs.d, Some("b"));
}

#[test]
fn split_attribute_of_parent_moving() {
    #[derive(PromAttire)]
    struct A {
        #[attire(split_attribute_of = "b")]
        c: Option<String>,
        #[attire(split_attribute_of = "b")]
        d: Option<String>,
    }
    let input = quote! {
        #[b = "b"]
        struct S {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.c, Some("b".to_owned()));
    assert_eq!(attrs.d, Some("b".to_owned()));
}

#[test]
fn split_attribute_of_children() {
    #[derive(PromAttire)]
    struct A<'a> {
        #[attire(split_attribute_of = "b")]
        c: Option<&'a str>,
        #[attire(split_attribute_of = "b")]
        d: Option<&'a str>,
        #[attire(split_attribute_of = "b")]
        e: Option<&'a str>,
    }
    let input = quote! {
        #[b(c = "c", d = "d")]
        struct S {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.c, Some("c"));
    assert_eq!(attrs.d, Some("d"));
    assert_eq!(attrs.e, None);
}

#[test]
fn split_attribute_and_normal() {
    #[derive(PromAttire)]
    struct A<'a> {
        #[attire(split_attribute_of = "b")]
        c: Option<&'a str>,
        #[attire(attribute = "c")]
        d: Option<&'a str>,
    }
    let input = quote! {
        #[b = "b"]
        #[c = "c"]
        struct S {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.c, Some("b"));
    assert_eq!(attrs.d, Some("c"));
}

#[test]
fn split_attribute_with_rename() {
    #[derive(PromAttire)]
    struct A<'a> {
        #[attire(split_attribute_of = "b", attribute = "d")]
        c: Option<&'a str>,
    }
    let input = quote! {
        #[b(d = "b")]
        struct S {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.c, Some("b"));
}
