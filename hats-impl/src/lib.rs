#![recursion_limit = "128"]

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
                "Vec" => quote! {
                    let mut #ident = Vec::new();
                },
                "Option" => quote! {
                    let mut #ident = None;
                },
                "bool" => quote! {
                    let mut #ident = false;
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

fn match_field(field: &syn::Field, config: &Config, lifetime: &syn::Lifetime) -> Tokens {
    let str_type = syn::parse_type(quote! { Option<&#lifetime str> }.as_str()).unwrap();
    let vec_str_type = syn::parse_type(quote! { Vec<&#lifetime str> }.as_str()).unwrap();
    let scope = config.scope.unwrap_or("");
    let ident = field.ident.as_ref().unwrap();
    let ident_str = ident.as_ref();
    if field.ty == syn::parse_type("bool").unwrap() {
        quote! {
            ::syn::MetaItem::NameValue(ref ident, ref value)
                if ident.as_ref() == #ident_str => {
                    #ident = match *value {
                        ::syn::Lit::Bool(value) => value,
                        ::syn::Lit::Str(ref value, _) => {
                            value.parse().unwrap_or_else(|err| {
                                panic!(
                                    "Parsing attribute value {:?} for {}({}) failed: {}",
                                    value, #scope, ident.as_ref(), err)
                            })
                        }
                        _ => {
                            panic!(
                                "Unexpected attribute literal value {:?} for {}({}), expected {}",
                                value, #scope, ident.as_ref(), "bool")
                        }
                    }
                }

            ::syn::MetaItem::Word(ref ident)
                if ident.as_ref() == #ident_str => {
                    #ident = true;
                }
        }
    } else if field.ty == str_type {
        quote! {
            ::syn::MetaItem::NameValue(ref ident, ref value)
                if ident.as_ref() == #ident_str => {
                    match *value {
                        ::syn::Lit::Str(ref value, _) => {
                            #ident = Some(value.as_ref())
                        }
                        _ => {
                            panic!(
                                "Unexpected attribute literal value {:?} for {}({}), expected {}",
                                value, #scope, ident.as_ref(), "bool")
                        }
                    }
                }
        }
    } else if field.ty == vec_str_type {
        quote! {
            ::syn::MetaItem::NameValue(ref ident, ref value)
                if ident.as_ref() == #ident_str => {
                    match *value {
                        ::syn::Lit::Str(ref value, _) => {
                            #ident.push(value.as_ref());
                        }
                        _ => {
                            panic!(
                                "Unexpected attribute literal value {:?} for {}({}), expected {}",
                                value, #scope, ident.as_ref(), "bool")
                        }
                    }
                }
        }
    } else {
        quote! {
            ::syn::MetaItem::NameValue(ref ident, ref value)
                if ident.as_ref() == #ident_str => {
                    #ident = match *value {
                        ::syn::Lit::Str(ref value, _) => {
                            Some(value.parse().unwrap_or_else(|err| {
                                panic!(
                                    "Parsing attribute value {:?} for {}({}) failed: {}",
                                    value, #scope, ident.as_ref(), err)
                            }))
                        }
                        _ => {
                            panic!(
                                "Unexpected attribute literal value {:?} for {}({}), expected {}",
                                value, #scope, ident.as_ref(), "bool")
                        }
                    }
                }
        }
    }
}

fn match_loop<I: Iterator<Item=Tokens>>(matches: I, config: &Config) -> Tokens {
    if let Some(scope) = config.scope {
        quote! {
            for attr in attrs {
                if let ::syn::MetaItem::List(ref ident, ref values) = attr.value {
                    if ident == #scope {
                        for value in values {
                            if let ::syn::NestedMetaItem::MetaItem(ref item) = *value {
                                match *item {
                                    #(#matches)*
                                    ref item => {
                                        panic!("Unexpected attribute {:?}", item);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        quote! {
            for attr in attrs {
                match *attr {
                    #(#matches)*
                    ref item => {
                        panic!("Unexpected attribute {:?}", item);
                    }
                }
            }
        }
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
    let field_matches = fields.iter().map(|field| match_field(field, config, lifetime));
    let match_loop = match_loop(field_matches, config);
    let write_fields = fields.iter().map(write_field);
    quote! {
        impl<'a> From<&'a [::syn::Attribute]> for #ident<'a> {
            fn from(attrs: &[::syn::Attribute]) -> #ident {
                #(#setup_fields)*
                #match_loop
                #ident {
                    #(#write_fields)*
                }
            }
        }
    }
}
