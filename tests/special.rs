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
fn special_bool_and_string() {
    // using renaming should be possible to support one attribute having a
    // value and being a bool
    #[derive(PromAttire, Debug, PartialEq)]
    struct A<'a> {
        a: Option<&'a str>,
        #[attire(attribute = "a")]
        b: Option<bool>,
    }
    {
        // word on it's own should be a bool
        let ast = syn::parse_derive_input("#[a] struct C {}").unwrap();
        let attrs = A::from(ast.attrs.as_slice());
        assert_eq!(attrs,
                   A {
                       a: None,
                       b: Some(true),
                   });
    }
    {
        // word with value should be parsed
        let ast = syn::parse_derive_input(r#"#[a = "c"] struct C {}"#)
            .unwrap();
        let attrs = A::from(ast.attrs.as_slice());
        assert_eq!(attrs,
                   A {
                       a: Some("c"),
                       b: None,
                   });
    }
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
    A::from(ast.attrs.as_slice());
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
    A::from(ast.attrs.as_slice());
}
