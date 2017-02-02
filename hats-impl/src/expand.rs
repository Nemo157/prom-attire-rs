use syn::{ self, DeriveInput };
use quote::Tokens;

use { Config, Result };

fn docs_field(field: &syn::Field, lifetime: &syn::Lifetime) -> Result<Tokens> {
    let expected_type = syn::parse_type(quote! { Vec<&#lifetime str> }.as_str()).unwrap();
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

fn setup_field(field: &syn::Field, lifetime: &syn::Lifetime) -> Result<Tokens> {
    let ident = field.ident.as_ref().unwrap();
    if ident.as_ref() == "docs" {
        return docs_field(field, lifetime);
    }
    Ok(match field.ty {
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
                _ => return Err("bad type".into()),
            }
        }
        _ => return Err("bad type".into()),
    })
}

fn write_field(field: &syn::Field) -> Tokens {
    let ident = field.ident.as_ref().unwrap();
    quote! {
        #ident: #ident,
    }
}

fn match_field(field: &syn::Field, config: &Config, lifetime: &syn::Lifetime) -> Tokens {
    let ident = field.ident.as_ref().unwrap();
    if ident.as_ref() == "docs" {
        return quote!();
    }
    let ident_str = ident.as_ref();
    let inner_ty = if let syn::Ty::Path(_, ref path) = field.ty {
        let segment = &path.segments[0];
        if segment.ident.as_ref() == "Option" {
            if let syn::PathParameters::AngleBracketed(ref parameters) = segment.parameters {
                Some(&parameters.types[0])
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };
    let str_type = syn::parse_type(quote! { Option<&#lifetime str> }.as_str()).unwrap();
    let vec_str_type = syn::parse_type(quote! { Vec<&#lifetime str> }.as_str()).unwrap();
    let scope = config.scope.unwrap_or("");
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
    } else if field.ty == syn::parse_type("Option<char>").unwrap() {
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
                    match *value {
                        ::syn::Lit::Str(ref value, _) => {
                            #ident = Some(<#inner_ty as ::std::str::FromStr>::from_str(value).unwrap_or_else(|err| {
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

pub fn expand(ast: &DeriveInput, config: &Config) -> Result<Tokens> {
    let fields = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => fields,
        _ => panic!("bad body"),
    };
    assert!(ast.generics.lifetimes.len() == 1);
    assert!(ast.generics.lifetimes[0].bounds.is_empty());
    assert!(ast.generics.ty_params.is_empty());
    let lifetime = &ast.generics.lifetimes[0].lifetime;
    let ident = &ast.ident;
    let setup_fields = fields.iter().map(|field| setup_field(field, lifetime)).collect::<Result<Vec<Tokens>>>()?;
    let field_matches = fields.iter().map(|field| match_field(field, config, lifetime));
    let match_loop = match_loop(field_matches, config);
    let write_fields = fields.iter().map(write_field);
    Ok(quote! {
        impl<'a> From<&'a [::syn::Attribute]> for #ident<'a> {
            fn from(attrs: &[::syn::Attribute]) -> #ident {
                #(#setup_fields)*
                #match_loop
                #ident {
                    #(#write_fields)*
                }
            }
        }
    })
}
