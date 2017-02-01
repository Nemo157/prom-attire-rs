extern crate syn;
#[macro_use]
extern crate quote;

use syn::DeriveInput;
use quote::Tokens;

pub struct Config<'a> {
    pub scope: Option<&'a str>,
}

pub fn derive(input: &str, config: Config) -> String {
    let ast = syn::parse_derive_input(input).unwrap();
    expand(&ast, &config).to_string()
}

fn docs_field(field: &syn::Field, lifetime: &syn::Lifetime) -> Tokens {
    let expected_type = syn::parse_type(quote! { Vec<&#lifetime str> }.as_str()).unwrap();
    if field.ty != expected_type {
        panic!("bad doc type");
    }
    quote! {
        let docs = attrs.iter()
            .filter(|a| a.is_sugared_doc)
            .map(|a| match a.value {
                ::syn::MetaItem::NameValue(_, ::syn::Lit::Str(ref doc, _)) => doc,
                _ => unreachable!(),
            })
            .map(|line| line.trim_left_matches('/').trim())
            .collect();
    }
}

fn setup_field(field: &syn::Field, lifetime: &syn::Lifetime) -> Tokens {
    let ident = field.ident.as_ref().unwrap();
    if ident.as_ref() == "docs" {
        return docs_field(field, lifetime);
    }
    match field.ty {
        syn::Ty::Path(None, ref path) => {
            assert!(!path.global);
            assert!(path.segments.len() == 1);
            match path.segments[0].ident.as_ref() {
                "Option" => quote! {
                    let #ident = None;
                },
                "bool" => quote! {
                    let #ident = false;
                },
                _ => panic!("bad type"),
            }
        }
        _ => panic!("bad type"),
    }
}

fn write_field(field: &syn::Field) -> Tokens {
    let ident = field.ident.as_ref().unwrap();
    quote! {
        #ident: #ident,
    }
}

fn expand(ast: &DeriveInput, config: &Config) -> Tokens {
    let fields = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => fields,
        _ => panic!("bad body"),
    };
    assert!(ast.generics.lifetimes.len() == 1);
    assert!(ast.generics.lifetimes[0].bounds.is_empty());
    assert!(ast.generics.ty_params.is_empty());
    let lifetime = &ast.generics.lifetimes[0].lifetime;
    let ident = &ast.ident;
    let setup_fields = fields.iter().map(|field| setup_field(field, lifetime));
    let write_fields = fields.iter().map(write_field);
    quote! {
        impl<'a> From<&'a [::syn::Attribute]> for #ident<'a> {
            fn from(attrs: &[::syn::Attribute]) -> #ident {
                #(#setup_fields)*
                #ident {
                    #(#write_fields)*
                }
            }
        }
    }
}

/*
::syn::MetaItem::NameValue(ref ident, ref value)
    if ident.as_ref() == stringify!($field) => {
        $name.$field = match *value {
            ::syn::Lit::Bool(value) => value,
            ::syn::Lit::Str(ref value, _) => {
                value.parse().unwrap_or_else(|err| {
                    panic!(
                        "Parsing attribute value {:?} for {}({}) failed: {}",
                        value, stringify!($scope), ident.as_ref(), err)
                })
            }
            _ => {
                panic!(
                    "Unexpected attribute literal value {:?} for {}({}), expected {}",
                    value, stringify!($scope), ident.as_ref(), "bool")
            }
        }
    }

::syn::MetaItem::Word(ref ident)
    if ident.as_ref() == stringify!($field) => {
        $name.$field = true;
    }

for attr in attrs {
    if let ::syn::MetaItem::List(ref ident, ref values) = attr.value {
        if ident == stringify!($scope) {
            for value in values {
                if let ::syn::NestedMetaItem::MetaItem(ref item) = *value {
                    match *item {
                        $($m)*
                        ref item => {
                            panic!("Unexpected attribute {:?}", item);
                        }
                    }
                }
            }
        }
    }
}
*/
