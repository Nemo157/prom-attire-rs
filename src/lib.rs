extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate prom_attire_bootstrap;
extern crate prom_attire_impl;

#[derive(PromAttireBootstrap)]
struct Attributes<'a> {
    scope: Option<&'a str>,
    docs: Option<&'a str>,
}

#[proc_macro_derive(PromAttire, attributes(attire))]
pub fn app(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = input.to_string();

    let ast = match syn::parse_derive_input(&input) {
        Ok(ast) => ast,
        Err(err) => {
            println!("{}", err);
            panic!("Internal error in prom-attire (probably)");
        }
    };

    let attrs = match Attributes::try_from(ast.attrs.as_slice()) {
        Ok(attrs) => attrs,
        Err(errs) => {
            for err in errs {
                println!("{}", err);
            }
            panic!("Invalid attributes specified for prom-attire macro");
        }
    };

    let config = prom_attire_impl::Config {
        scope: attrs.scope,
        docs: attrs.docs,
    };

    let expanded = match prom_attire_impl::derive(&input, config) {
        Ok(expanded) => expanded,
        Err(err) => {
            println!("{}", err);
            panic!("Expanding prom-attire failed");
        }
    };

    match expanded.parse() {
        Ok(parsed) => parsed,
        Err(err) => {
            println!("{:?}", err);
            panic!("Internal error in prom-attire");
        }
    }
}
