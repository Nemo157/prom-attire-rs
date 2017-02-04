#[macro_use]
extern crate prom_attire;
#[macro_use]
extern crate quote;
extern crate syn;

#[test]
fn docs_zero() {
    #[derive(PromAttire)]
    #[attire(docs = "docs")]
    struct A<'a> {
        docs: Vec<&'a str>,
    }
    // quote! desugars docs into normal #[doc = " Some docs"] attributes
    let input = quote! {
        /// Some docs
        /// For this struct
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.docs, [" Some docs", " For this struct"]);
}

#[test]
fn docs_sugared() {
    #[derive(PromAttire)]
    #[attire(docs = "docs")]
    struct A<'a> {
        docs: Vec<&'a str>,
    }
    let input = "
        /// Some docs
        /// For this struct
        struct C {}
    ";
    let ast = syn::parse_derive_input(input).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.docs, [" Some docs", " For this struct"]);
}
