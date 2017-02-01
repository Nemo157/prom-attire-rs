extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate hats_bootstrap;
extern crate hats_impl;

#[derive(FromAttributesBootstrap)]
struct Attributes<'a> {
    scope: Option<&'a str>,
}

#[proc_macro_derive(FromAttributes, attributes(hats))]
pub fn app(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = input.to_string();

    let ast = syn::parse_derive_input(&input).unwrap();
    let attrs = Attributes::from(ast.attrs.as_slice());

    let config = hats_impl::Config {
        scope: attrs.scope
    };

    hats_impl::derive(&input, config).parse().unwrap()
}
