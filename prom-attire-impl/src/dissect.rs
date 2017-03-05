//! Takes an input struct and extracts all the details necessary to generate
//! the From<&[Attribute]> implementation

use std::collections::HashMap;
use std::collections::hash_map::Entry;

use syn;

use errors::*;
use tmp::{TryFrom, TryInto};
use {Config, FieldConfig};

#[derive(Debug)]
pub struct Struct<'a> {
    pub ast: &'a syn::DeriveInput,
    pub docs: Option<Field<'a>>,
    pub lifetime: Option<&'a syn::Lifetime>,
    pub fields: Vec<Field<'a>>,
    pub split_fields: Vec<SplitFields<'a>>,
}

#[derive(Debug)]
pub struct SplitFields<'a> {
    pub parent: &'a str,
    pub ty: Wrapper<'a>,
    pub syn_ty: &'a syn::Ty,
    pub fields: Vec<Field<'a>>,
}

#[derive(Debug)]
pub struct Field<'a> {
    pub ast: &'a syn::Field,
    pub ident: &'a syn::Ident,
    pub attribute: &'a str,
    pub ty: Wrapper<'a>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Wrapper<'a> {
    None(Ty<'a>),
    Option(Ty<'a>),
    Vec(Ty<'a>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Ty<'a> {
    Literal(Lit),
    Custom(&'a syn::Ty),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Lit {
    Bool,
    Char,
    Int(syn::IntTy),
    Str,
    ByteStr,
    Float(syn::FloatTy),
}

impl<'a> TryFrom<(&'a syn::DeriveInput, &'a Config<'a>)> for Struct<'a> {
    type Err = Error;

    fn try_from((ast, config): (&'a syn::DeriveInput, &'a Config<'a>))
        -> Result<Self> {
        let syn_fields = match ast.body {
            syn::Body::Struct(syn::VariantData::Struct(ref fields)) => fields,
            _ => bail!(ErrorKind::StructBody),
        };

        if !ast.generics.ty_params.is_empty() {
            bail!(ErrorKind::TyParams(ast.generics.ty_params.clone()));
        }

        if ast.generics.lifetimes.len() > 1 {
            bail!(ErrorKind::Lifetimes(ast.generics.lifetimes.clone()));
        }

        let lifetime = ast.generics
            .lifetimes
            .iter()
            .next()
            .map(|l| &l.lifetime);

        let docs_field = config.docs.and_then(|docs| {
            syn_fields.iter()
                .find(|field| field.ident.as_ref().unwrap().as_ref() == docs)
        });

        // Can't use Option::map because there's no simple transform
        // Option<Result<T>> -> Result<Option<T>> for the ? op to apply to
        let docs = match docs_field {
            Some(field) => {
                let config = (config.parse_field_config)(field.attrs
                    .as_slice());
                let docs: Field = (field, config).try_into()?;
                if docs.ty != Wrapper::Vec(Ty::Literal(Lit::Str)) {
                    bail!(ErrorKind::DocsTy(field.clone()));
                }
                Some(docs)
            }
            None => None,
        };

        let mut fields = Vec::with_capacity(syn_fields.len());
        let mut split_fields = HashMap::new();

        let parse_field_config = &config.parse_field_config;

        for syn_field in syn_fields {
            if Some(syn_field.ident.as_ref().unwrap().as_ref()) ==
               config.docs {
                continue;
            }
            let field_config = parse_field_config(syn_field.attrs.as_slice());
            match field_config.split_attribute_of {
                None => fields.push((syn_field, field_config).try_into()?),
                Some(parent) => {
                    let field: Field = (syn_field, field_config).try_into()?;
                    match split_fields.entry(parent) {
                        Entry::Occupied(mut entry) => {
                            let split: &mut SplitFields = entry.get_mut();
                            if split.ty != field.ty {
                                bail!(ErrorKind::SplitFieldTys(split.parent.to_owned(), split.syn_ty.clone(), syn_field.clone()));
                            }
                            split.fields.push(field);
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(SplitFields {
                                parent: parent,
                                ty: field.ty.clone(),
                                syn_ty: &syn_field.ty,
                                fields: vec![field],
                            });
                        }
                    }
                }
            }
        }

        Ok(Struct {
            ast: ast,
            docs: docs,
            lifetime: lifetime,
            fields: fields,
            split_fields: split_fields.into_iter().map(|(_, v)| v).collect(),
        })
    }
}

impl<'a> TryFrom<(&'a syn::Field, FieldConfig<'a>)> for Field<'a> {
    type Err = Error;

    fn try_from((ast, config): (&'a syn::Field, FieldConfig<'a>))
        -> Result<Self> {
        let ident = ast.ident.as_ref().unwrap();
        Ok(Field {
            ast: ast,
            ident: ident,
            attribute: config.attribute.unwrap_or(ident.as_ref()),
            ty: (&ast.ty).try_into()
                .chain_err(|| ErrorKind::Field(ast.clone()))?,
        })
    }
}

impl<'a> TryFrom<&'a syn::Ty> for Wrapper<'a> {
    type Err = Error;

    fn try_from(ty: &'a syn::Ty) -> Result<Self> {
        let path = match *ty {
            syn::Ty::Path(None, ref path) => path,
            _ => bail!(ErrorKind::TyWrapper(ty.clone())),
        };

        if path.global || path.segments.len() != 1 {
            bail!(ErrorKind::TyWrapper(ty.clone()));
        }

        let segment = &path.segments[0];
        Ok(match segment.ident.as_ref() {
            "Option" => Wrapper::Option(ty_try_from_option_or_vec(&segment.parameters, &ty)?),
            "Vec" => {
                Wrapper::Vec(ty_try_from_option_or_vec(&segment.parameters,
                                                       &ty)?)
            }
            "bool" => Wrapper::None(Ty::Literal(Lit::Bool)),
            _ => bail!(ErrorKind::TyWrapper(ty.clone())),
        })
    }
}

impl<'a> Wrapper<'a> {
    pub fn inner(&self) -> &Ty<'a> {
        match *self {
            Wrapper::None(ref ty) => ty,
            Wrapper::Option(ref ty) => ty,
            Wrapper::Vec(ref ty) => ty,
        }
    }
}

fn ty_try_from_option_or_vec<'a>(
    p: &'a syn::PathParameters,
    ty: &'a syn::Ty
) -> Result<Ty<'a>> {
    let data = if let syn::PathParameters::AngleBracketed(ref data) = *p {
        data
    } else {
        bail!(ErrorKind::TyWrapper(ty.clone()));
    };

    if !data.lifetimes.is_empty() || !data.bindings.is_empty() {
        bail!(ErrorKind::TyWrapper(ty.clone()));
    }

    if data.types.len() != 1 {
        bail!(ErrorKind::TyWrapper(ty.clone()));
    }

    (&data.types[0]).try_into()
}

impl<'a> TryFrom<&'a syn::Ty> for Ty<'a> {
    type Err = Error;

    fn try_from(ty: &'a syn::Ty) -> Result<Self> {
        Ok(match *ty {
            syn::Ty::Path(None, ref path) => {
                if path.segments.is_empty() {
                    bail!(ErrorKind::Ty(ty.clone()));
                }

                match path.segments[0].ident.as_ref() {
                    "bool" => Ty::Literal(Lit::Bool),
                    "char" => Ty::Literal(Lit::Char),
                    "u8" => Ty::Literal(Lit::Int(syn::IntTy::U8)),
                    "i8" => Ty::Literal(Lit::Int(syn::IntTy::I8)),
                    "u16" => Ty::Literal(Lit::Int(syn::IntTy::U16)),
                    "i16" => Ty::Literal(Lit::Int(syn::IntTy::I16)),
                    "u32" => Ty::Literal(Lit::Int(syn::IntTy::U32)),
                    "i32" => Ty::Literal(Lit::Int(syn::IntTy::I32)),
                    "u64" => Ty::Literal(Lit::Int(syn::IntTy::U64)),
                    "i64" => Ty::Literal(Lit::Int(syn::IntTy::I64)),
                    "usize" => Ty::Literal(Lit::Int(syn::IntTy::Usize)),
                    "isize" => Ty::Literal(Lit::Int(syn::IntTy::Isize)),
                    "f32" => Ty::Literal(Lit::Float(syn::FloatTy::F32)),
                    "f64" => Ty::Literal(Lit::Float(syn::FloatTy::F64)),
                    _ => Ty::Custom(ty),
                }
            }
            syn::Ty::Rptr(_, ref ty) => {
                if ty.mutability != syn::Mutability::Immutable {
                    bail!(ErrorKind::TyRef(ty.ty.clone()));
                }
                if ty.ty == syn::parse_type("str").unwrap() {
                    Ty::Literal(Lit::Str)
                } else if ty.ty == syn::parse_type("[u8]").unwrap() {
                    Ty::Literal(Lit::ByteStr)
                } else {
                    bail!(ErrorKind::TyRef(ty.ty.clone()))
                }
            }
            _ => bail!(ErrorKind::Ty(ty.clone())),
        })
    }
}

impl<'a> Ty<'a> {
    pub fn lit(&self) -> Option<Lit> {
        match *self {
            Ty::Literal(lit) => Some(lit),
            Ty::Custom(_) => None,
        }
    }
}
