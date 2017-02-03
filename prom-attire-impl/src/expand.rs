use syn;
use quote::{Tokens, ToTokens};

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

fn match_error(scope: &Option<&str>, ty: &Ty) -> Tokens {
    if let Some(ref scope) = *scope {
        quote! {
            _ => panic!(
                concat!(
                    "Unexpected attribute literal '{:?}' for ",
                    stringify!(#scope),
                    "({}), expected ",
                    stringify!(#ty)),
                value,
                ident.as_ref())
        }
    } else {
        quote! {
            _ => panic!(
                concat!(
                    "Unexpected attribute literal '{:?}' for {}, expected ",
                    stringify!(#ty)),
                value,
                ident.as_ref())
        }
    }
}

fn match_parse(ty: &Ty) -> Tokens {
    match *ty {
        Ty::Literal(Lit::Str) => {
            quote! {
                value.as_ref()
            }
        }
        Ty::Literal(Lit::Char) => {
            quote! {
                if value.len() != 1 {
                    panic!(
                        "Parsing attribute value {:?} for {} failed: {}",
                        value, ident.as_ref(), "expected one character");
                }
                value.chars().next().unwrap()
            }
        }
        Ty::Literal(Lit::ByteStr) => {
            quote! {
                assert!(::std::ascii::AsciiExt::is_ascii(value.as_str()));
                value.as_bytes()
            }
        }
        ref ty => {
            quote! {
                <#ty as ::std::str::FromStr>::from_str(value)
                    .unwrap_or_else(|err| {
                        panic!(
                            "Parsing attribute value {:?} for {} failed: {}",
                            value, ident.as_ref(), err)
                    })
            }
        }
    }
}

fn match_literal(ty: &Ty, lit: Lit) -> Tokens {
    match lit {
        Lit::Bool => {
            quote! { ::syn::Lit::Bool(value) => { value } }
        }
        Lit::Char => {
            quote! { ::syn::Lit::Char(value) => { value } }
        }
        Lit::Int(_) => {
            quote! { ::syn::Lit::Int(value, _) => { value as #ty } }
        }
        Lit::Str => {
            // Handle as a parse
            quote!()
        }
        Lit::ByteStr => {
            quote! { ::syn::Lit::ByteStr(ref value, _) => { value.as_ref() } }
        }
        Lit::Float(_) => {
            quote! {
                ::syn::Lit::Float(ref value, _) => {
                    <#ty as ::std::str::FromStr>::from_str(value.as_str())
                        .unwrap_or_else(|err| {
                            panic!(
                                "Parsing attribute value {:?} for {} failed: {}",
                                value, ident.as_ref(), err)
                        })
                }
            }
        }
    }
}

fn match_write(ident: &syn::Ident, ty: &Wrapper) -> Tokens {
    match *ty {
        Wrapper::Vec(_) => {
            quote! {
                #ident.push(value);
            }
        }
        Wrapper::Option(_) => {
            quote! {
                #ident = Some(value);
            }
        }
        Wrapper::None(_) => {
            quote! {
                #ident = value;
            }
        }
    }
}

fn match_special(ident: &syn::Ident, ty: &Wrapper) -> Tokens {
    let ident_str = ident.as_ref();
    let write = match_write(ident, ty);
    match *ty.inner() {
        Ty::Literal(Lit::Bool) => {
            quote! {
                ::syn::MetaItem::Word(ref ident)
                    if ident.as_ref() == #ident_str => {
                        let value = true;
                        #write
                    }
            }
        }
        _ => quote!(),
    }
}

fn match_field(field: &Field, config: &Config) -> Tokens {
    let ident = &field.ident;
    let ident_str = ident.as_ref();
    let error = match_error(&config.scope, field.ty.inner());
    let parse = match_parse(field.ty.inner());
    let literal =
        field.ty.inner().lit().map(|lit| match_literal(field.ty.inner(), lit));
    let write = match_write(ident, &field.ty);
    let special = match_special(ident, &field.ty);
    quote! {
        ::syn::MetaItem::NameValue(ref ident, ref value)
            if ident.as_ref() == #ident_str => {
                let value = match *value {
                    ::syn::Lit::Str(ref value, _) => {
                        #parse
                    }
                    #literal
                    #error
                };
                #write
            }
        #special
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
                                        println!(
                                            "Unexpected attribute under '{}' ({:?})",
                                            #scope, item);
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
                    _ => {
                        // Ignore it, we're unscoped so no control over what
                        // appears
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
    let write_fields = strukt.fields.iter().map(write_field);
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
            Ty::Literal(Lit::Str) => tokens.append("str"),
            Ty::Literal(Lit::ByteStr) => tokens.append("bytestr"),
            Ty::Literal(Lit::Float(ty)) => tokens.append(&ty.to_string()),
            Ty::Custom(ref ty) => ty.to_tokens(tokens),
        }
    }
}
