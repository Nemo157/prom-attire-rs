#[macro_use]
extern crate prom_attire;
#[macro_use]
extern crate quote;
extern crate syn;

#[test]
fn docs() {
    #[derive(PromAttire)]
    #[attire(docs = "docs")]
    struct A<'a> {
        docs: Vec<&'a str>,
    }
    let input = quote! {
        ///Some docs
        ///For this struct
        struct C {}
    };
    let ast = syn::parse_derive_input(input.as_str()).unwrap();
    let attrs = A::from(ast.attrs.as_slice());
    assert_eq!(attrs.docs, ["Some docs", "For this struct"]);
}
