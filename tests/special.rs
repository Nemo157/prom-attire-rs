#[macro_use]
extern crate prom_attire;
#[macro_use]
extern crate quote;
extern crate syn;

#[test]
fn special_bool_unwrapped() {
    // bool should support being an unwrapped type, defaulting to false
    #[derive(PromAttire)]
    struct A {
        b: bool,
    }
    let input = quote! {
        #[b = "true"]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, true);
}

#[test]
fn special_bool_word() {
    // bool should support being just a word without setting a value to it
    #[derive(PromAttire)]
    struct A {
        b: Option<bool>,
    }
    let input = quote! {
        #[b]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.b, Some(true));
}

#[test]
fn unscoped_extra_attributes_are_ignored() {
    #[derive(PromAttire)]
    struct A {}
    let input = quote! {
        #[b = "false"]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    // Should not panic here just because there's an extra attribute
    let attrs = A::from(ast.attrs.as_slice());
}

#[test]
fn scoped_extra_attributes_warn() {
    #[derive(PromAttire)]
    #[attire(scope = "carrot")]
    struct A {}
    let input = quote! {
        #[carrot(b = "false")]
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    // Should not panic here just because there's an extra attribute
    // Not sure how warnings should be output actually.......
    let attrs = A::from(ast.attrs.as_slice());
}
