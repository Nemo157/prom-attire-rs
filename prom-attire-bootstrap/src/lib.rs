extern crate proc_macro;
extern crate syn;
extern crate error_chain;

extern crate prom_attire_impl;

use error_chain::ChainedError;

#[proc_macro_derive(PromAttireBootstrap, attributes(attire_bootstrap))]
pub fn app(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = &input.to_string();

    let config = prom_attire_impl::Config {
        scope: Some("attire"),
        docs: None,
        parse_field_config: &|attrs| {
            attrs.first()
                .and_then(|attr| match attr.value {
                    syn::MetaItem::List(ref ident, ref values)
                        if ident == "attire_bootstrap" => values.first(),
                    _ => None,
                })
                .and_then(|value| match *value {
                    syn::NestedMetaItem::MetaItem(ref item) => Some(item),
                    _ => None,
                })
                .and_then(|item| match *item {
                    syn::MetaItem::Word(ref ident) => Some(ident),
                    _ => None,
                })
                .and_then(|item| Some(match item.as_ref() {
                    "field_default" => {
                        prom_attire_impl::FieldConfig {
                            attribute: None,
                            split_attribute_of: None,
                            default: prom_attire_impl::Defaulted::To("prom_attire_impl::Defaulted::Nope".to_owned()),
                            flag_value: Some("prom_attire_impl::Defaulted::Yep"),
                        }
                    }
                    _ => { return None; }
                }))
                .unwrap_or_else(|| prom_attire_impl::FieldConfig::default())
        }
    };

    let expanded = match prom_attire_impl::derive(input, &config) {
        Ok(expanded) => expanded,
        Err(err) => {
            println!("{}", err.display());
            panic!("Expanding prom-attire-bootstrap failed");
        }
    };

    match expanded.parse() {
        Ok(parsed) => parsed,
        Err(err) => {
            println!("{:?}", err);
            panic!("Internal error in prom-attire-bootstrap");
        }
    }
}
