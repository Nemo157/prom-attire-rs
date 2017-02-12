use syn;
use quote::{Tokens, ToTokens};

use dissect::{Struct, Field, Wrapper, Ty, Lit};
use Config;

struct Context<'a> {
    config: &'a Config<'a>,
    strukt_ty: &'a syn::Ident,
    /// A lifetime, either one from the struct or a new generated one
    life: Tokens,
    /// A lifetime bound, using the lifetime `life`
    life_bound: Tokens,
    /// A lifetime bound if the struct has a lifetime bound, if it is Some it
    /// uses the lifetime from `life`
    opt_life_bound: Option<Tokens>,
    scope_lit: Tokens,
    error_ty: syn::Ident,
}

impl<'a> Context<'a> {
    fn new(strukt: &'a Struct, config: &'a Config) -> Context<'a> {
        let opt_life_bound = strukt.lifetime.as_ref().map(|l| quote!(<#l>));
        let life = strukt.lifetime
            .as_ref()
            .map(|l| quote!(#l))
            .unwrap_or_else(|| quote!('a));
        let life_bound = quote!(<#life>);

        Context {
            config: config,
            strukt_ty: &strukt.ast.ident,
            life: life,
            life_bound: life_bound,
            opt_life_bound: opt_life_bound,
            scope_lit: match config.scope {
                Some(ref scope) => quote!(Some(#scope)),
                None => quote!(None),
            },
            error_ty: syn::Ident::new(strukt.ast.ident.as_ref().to_string() +
                                      "FromAttributesError"),
        }
    }
}

fn setup_docs(field: &Field) -> Tokens {
    let ident = &field.ident;
    quote! {
        let #ident = attrs.iter()
            .filter_map(|a| {
                use ::syn::MetaItem::NameValue;
                if a.is_sugared_doc {
                    match a.value {
                        NameValue(_, ::syn::Lit::Str(ref doc, _)) =>
                            Some(doc.trim_left_matches("///")),
                        _ => None,
                    }
                } else {
                    match a.value {
                        NameValue(ref name, ::syn::Lit::Str(ref doc, _))
                            if name.as_ref() == "doc" => Some(doc.as_str()),
                        _ => None,
                    }
                }
            })
            .collect();
    }
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

fn match_error(ctx: &Context, ty: &Ty) -> Tokens {
    let scope_lit = &ctx.scope_lit;
    let error_ty = &ctx.error_ty;
    quote! {
        _ => {
            errors.push(#error_ty::LiteralTy {
                value: value,
                ty: stringify!(#ty),
                scope: #scope_lit,
                attr: ident.as_ref(),
            });
            continue;
        }
    }
}

fn match_parse(ctx: &Context, ty: &Ty) -> Tokens {
    let scope_lit = &ctx.scope_lit;
    let error_ty = &ctx.error_ty;
    match *ty {
        Ty::Literal(Lit::Str) => {
            quote! {
                value.as_ref()
            }
        }

        Ty::Literal(Lit::Char) => {
            quote! {
                if value.len() != 1 {
                    errors.push(#error_ty::Parsing {
                        value: value,
                        ty: stringify!(#ty),
                        scope: #scope_lit,
                        attr: ident.as_ref(),
                        err: Box::new(#error_ty::ParsingChar),
                    });
                    continue;
                }
                value.chars().next().unwrap()
            }
        }

        Ty::Literal(Lit::ByteStr) => {
            quote! {
                if ::std::ascii::AsciiExt::is_ascii(value.as_str()) {
                    value.as_bytes()
                } else {
                    errors.push(#error_ty::Parsing {
                        value: value,
                        ty: stringify!(#ty),
                        scope: #scope_lit,
                        attr: ident.as_ref(),
                        err: Box::new(#error_ty::ParsingByteStr),
                    });
                    continue;
                }
            }
        }

        ref ty => {
            quote! {
                match <#ty as ::std::str::FromStr>::from_str(value) {
                    Ok(value) => value,
                    Err(err) => {
                        errors.push(#error_ty::Parsing {
                            value: value,
                            ty: stringify!(#ty),
                            scope: #scope_lit,
                            attr: ident.as_ref(),
                            err: Box::new(err),
                        });
                        continue;
                    }
                }
            }
        }
    }
}

fn match_literal(ctx: &Context, ty: &Ty, lit: Lit) -> Tokens {
    let scope_lit = &ctx.scope_lit;
    let error_ty = &ctx.error_ty;
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
            // Handled as a parse
            quote!()
        }
        Lit::ByteStr => {
            quote! { ::syn::Lit::ByteStr(ref value, _) => { value.as_ref() } }
        }
        Lit::Float(_) => {
            quote! {
                ::syn::Lit::Float(ref value, _) => {
                    match <#ty as ::std::str::FromStr>::from_str(value.as_str()) {
                        Ok(value) => value,
                        Err(err) => {
                            errors.push(#error_ty::Parsing {
                                value: value,
                                ty: stringify!(#ty),
                                scope: #scope_lit,
                                attr: ident.as_ref(),
                                err: Box::new(err),
                            });
                            continue;
                        }
                    }
                }
            }
        }
    }
}

fn match_write(field: &Field, ty: &Wrapper) -> Tokens {
    let ident = &field.ident;
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

fn match_special(field: &Field, ty: &Wrapper) -> Tokens {
    let attribute = &field.attribute;
    let write = match_write(field, ty);
    match *ty.inner() {
        Ty::Literal(Lit::Bool) => {
            quote! {
                ::syn::MetaItem::Word(ref ident)
                    if ident.as_ref() == #attribute => {
                        let value = true;
                        #write
                    }
            }
        }
        _ => quote!(),
    }
}

fn match_field(ctx: &Context, field: &Field) -> Tokens {
    let attribute = &field.attribute;
    let error = match_error(ctx, field.ty.inner());
    let parse = match_parse(ctx, field.ty.inner());
    let literal = field.ty
        .inner()
        .lit()
        .map(|lit| match_literal(ctx, field.ty.inner(), lit));
    let write = match_write(field, &field.ty);
    let special = match_special(field, &field.ty);
    quote! {
        ::syn::MetaItem::NameValue(ref ident, ref value)
            if ident.as_ref() == #attribute => {
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
    ctx: &Context,
    matches: I
) -> Tokens {
    if let Some(scope) = ctx.config.scope {
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
    let ctx = Context::new(strukt, config);

    let setup_fields = strukt.fields
        .iter()
        .map(setup_field)
        .chain(strukt.docs.as_ref().map(setup_docs));
    let field_matches = strukt.fields
        .iter()
        .map(|field| match_field(&ctx, field));
    let match_loop = match_loop(&ctx, field_matches);
    let write_fields =
        strukt.fields.iter().chain(&strukt.docs).map(write_field);

    let Context { ref strukt_ty,
                  ref error_ty,
                  ref life,
                  ref life_bound,
                  ref opt_life_bound,
                  .. } = ctx;

    quote! {
        #[allow(dead_code)]
        #[derive(Debug)]
        enum #error_ty#life_bound {
            LiteralTy {
                value: &#life ::syn::Lit,
                ty: &'static str,
                scope: Option<&'static str>,
                attr: &#life str,
            },

            Parsing {
                value: &#life str,
                ty: &'static str,
                scope: Option<&'static str>,
                attr: &#life str,
                err: Box<::std::error::Error + #life>,
            },

            /// This is an internal error that will only be returned as part of
            /// the `err` field on `Parsing`
            ParsingChar,

            /// This is an internal error that will only be returned as part of
            /// the `err` field on `Parsing`
            ParsingByteStr,
        }

        impl#life_bound ::std::fmt::Display for #error_ty#life_bound {
            fn fmt(&self, mut w: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    #error_ty::LiteralTy { ref value, ref ty, ref scope, ref attr } => {
                        write!(
                            w,
                           "Unexpected attribute literal {:?} for {}, expected a {}",
                            value,
                            scope.map(|s| format!("{}({})", s, attr)).unwrap_or_else(|| attr.to_string()),
                            ty)
                    }

                    #error_ty::Parsing { ref value, ref ty, ref scope, ref attr, ref err } => {
                        write!(
                            w,
                           "Parsing attribute value {:?} into a {} for {} failed: {}",
                            value,
                            scope.map(|s| format!("{}({})", s, attr)).unwrap_or_else(|| attr.to_string()),
                            ty,
                            err)
                    }

                    #error_ty::ParsingChar => {
                        write!(w, "expected one character")
                    }

                    #error_ty::ParsingByteStr => {
                        write!(w, "non-ascii character")
                    }
                }
            }
        }

        impl#life_bound ::std::error::Error for #error_ty#life_bound {
            fn description(&self) -> &str {
                match *self {
                    #error_ty::LiteralTy { .. } => "Unexpected attribute literal",
                    #error_ty::Parsing { .. } => "Parsing attribute value failed",
                    #error_ty::ParsingChar => "expected one character",
                    #error_ty::ParsingByteStr => "non-ascii character",
                }
            }

            fn cause(&self) -> Option<&::std::error::Error> {
                match *self {
                    #error_ty::LiteralTy { .. } => None,
                    #error_ty::Parsing { ref err, .. } => Some(&**err),
                    #error_ty::ParsingChar => None,
                    #error_ty::ParsingByteStr => None,
                }
            }
        }

        impl#opt_life_bound #strukt_ty#opt_life_bound {
            #[allow(unused_mut)]
            fn try_from(attrs: &[::syn::Attribute])
                -> ::std::result::Result<#strukt_ty, ::std::vec::Vec<#error_ty>>
            {
                let mut errors = vec![];
                #(#setup_fields)*
                #match_loop
                if errors.is_empty() {
                    Ok(#strukt_ty {
                        #(#write_fields)*
                    })
                } else {
                    Err(errors)
                }
            }
        }

        impl<#life> From<&#life [::syn::Attribute]> for #strukt_ty#opt_life_bound {
            fn from(attrs: &[::syn::Attribute]) -> #strukt_ty {
                #strukt_ty::try_from(attrs).unwrap()
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
