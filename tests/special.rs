#[macro_use]
extern crate prom_attire;
#[macro_use]
extern crate quote;
extern crate syn;

#[test]
fn special_bool_unwrapped() {
    // bool should support being an unwrapped type, defaulting to false
    #[derive(FromAttributes)]
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
    #[derive(FromAttributes)]
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
