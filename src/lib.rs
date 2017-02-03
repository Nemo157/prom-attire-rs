extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate prom_attire_bootstrap;
extern crate prom_attire_impl;

#[derive(FromAttributesBootstrap)]
struct Attributes<'a> {
    scope: Option<&'a str>,
}

#[proc_macro_derive(FromAttributes, attributes(attire))]
pub fn app(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = input.to_string();

    let ast = syn::parse_derive_input(&input).unwrap();
    let attrs = Attributes::from(ast.attrs.as_slice());

    let config = prom_attire_impl::Config {
        scope: attrs.scope
    };

    prom_attire_impl::derive(&input, config).unwrap().parse().unwrap()
}
