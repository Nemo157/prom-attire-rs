extern crate proc_macro;
extern crate syn;

extern crate hats_impl;

#[proc_macro_derive(FromAttributesBootstrap)]
pub fn app(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let config = hats_impl::Config {
        scope: Some("hats"),
    };
    hats_impl::derive(&input.to_string(), config).unwrap().parse().unwrap()
}
