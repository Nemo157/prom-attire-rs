use syn;
use quote::{ Tokens, ToTokens };

use dissect::{Struct, Field, Wrapper, Ty, Lit};
use Config;
use errors::*;

// TODO: add a config to id this
fn docs_field(
    field: &syn::Field,
    lifetime: &Option<&syn::Lifetime>
) -> Result<Tokens> {
    let expected_type =
        syn::parse_type(quote! { Vec<&#lifetime str> }.as_str()).unwrap();
    if field.ty != expected_type {
        return Err("bad doc type".into());
    }
    Ok(quote! {
        let docs = attrs.iter()
            .filter(|a| a.is_sugared_doc)
            .map(|a| match a.value {
                ::syn::MetaItem::NameValue(_, ::syn::Lit::Str(ref doc, _)) => doc,
                _ => unreachable!(),
            })
            .map(|line| line.trim_left_matches('/').trim())
            .collect();
    })
}

fn setup_field(field: &Field) -> Tokens {
    let ident = &field.ident;
    match field.ty {
        Wrapper::Vec(_) => {
            quote! {
                let mut #ident = Vec::new();
            }
        }
        Wrapper::Option(_) => {
            quote! {
                let mut #ident = None;
            }
        }
        Wrapper::None(Ty::Literal(Lit::Bool)) => {
            quote! {
                let mut #ident = false;
            }
        }
        _ => unreachable!(),
    }
}

fn write_field(field: &Field) -> Tokens {
    let ident = &field.ident;
    quote! {
        #ident: #ident,
    }
}

fn match_field(field: &Field, config: &Config) -> Tokens {
    let ident = &field.ident;
    let ident_str = ident.as_ref();
    let scope = config.scope.unwrap_or("");
    match field.ty {
        Wrapper::Vec(_) => {
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
        }

        Wrapper::Option(Ty::Literal(Lit::Str)) => {
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
        }

        Wrapper::Option(Ty::Literal(Lit::ByteStr)) => {
            quote! {
                ::syn::MetaItem::NameValue(ref ident, ref value)
                    if ident.as_ref() == #ident_str => {
                        match *value {
                            ::syn::Lit::Str(ref value, _) => {
                                assert!(::std::ascii::AsciiExt::is_ascii(value.as_str()));
                                #ident = Some(value.as_bytes())
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

        Wrapper::Option(Ty::Literal(Lit::Char)) => {
            quote! {
                ::syn::MetaItem::NameValue(ref ident, ref value)
                    if ident.as_ref() == #ident_str => {
                        match *value {
                            ::syn::Lit::Str(ref value, _) => {
                                if value.len() != 1 {
                                    panic!(
                                        "Parsing attribute value {:?} for {}({}) failed: {}",
                                        value, #scope, ident.as_ref(),
                                        "expected one character");
                                }
                                #ident = Some(value.chars().next().unwrap());
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

        Wrapper::Option(ref ty) => {
            quote! {
                ::syn::MetaItem::NameValue(ref ident, ref value)
                    if ident.as_ref() == #ident_str => {
                        match *value {
                            ::syn::Lit::Str(ref value, _) => {
                                #ident = Some(<#ty as ::std::str::FromStr>::from_str(value).unwrap_or_else(|err| {
                                    panic!(
                                        "Parsing attribute value {:?} for {}({}) failed: {}",
                                        value, #scope, ident.as_ref(), err)
                                }));
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

        Wrapper::None(Ty::Literal(Lit::Bool)) => {
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
        }

        _ => unreachable!(),
    }
}

fn match_loop<I: Iterator<Item = Tokens>>(
    matches: I,
    config: &Config
) -> Tokens {
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
                match attr.value {
                    #(#matches)*
                    ref item => {
                        panic!("Unexpected attribute {:?}", item);
                    }
                }
            }
        }
    }
}

pub fn expand(strukt: &Struct, config: &Config) -> Tokens {
    let ident = &strukt.ast.ident;
    let setup_fields = strukt.fields
        .iter()
        .map(setup_field);
    let field_matches = strukt.fields
        .iter()
        .map(|field| match_field(field, config));
    let match_loop = match_loop(field_matches, config);
    let write_fields =
        strukt.fields.iter().map(write_field);
    let a = if strukt.lifetime.is_some() {
        quote!(<'a>)
    } else {
        quote!()
    };
    quote! {
        impl<'a> From<&'a [::syn::Attribute]> for #ident#a {
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


impl<'a> ToTokens for Ty<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        match *self {
            Ty::Literal(Lit::Bool) => tokens.append("bool"),
            Ty::Literal(Lit::Char) => tokens.append("char"),
            Ty::Literal(Lit::Int(ty)) => tokens.append(&ty.to_string()),
            Ty::Literal(Lit::Str) => panic!("str"),
            Ty::Literal(Lit::ByteStr) => panic!("bytestr"),
            Ty::Literal(Lit::Float(ty)) => tokens.append(&ty.to_string()),
            Ty::Custom(ref ty) => ty.to_tokens(tokens),
        }
    }
}
