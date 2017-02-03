//! Takes an input struct and extracts all the details necessary to generate
//! the From<&[Attribute]> implementation

use syn;

use errors::*;
use tmp::{TryFrom, TryInto};

pub struct Struct<'a> {
    pub ast: &'a syn::DeriveInput,
    pub lifetime: Option<&'a syn::Lifetime>,
    pub fields: Vec<Field<'a>>,
}

pub struct Field<'a> {
    pub ast: &'a syn::Field,
    pub ident: &'a syn::Ident,
    pub ty: Wrapper<'a>,
}

pub enum Wrapper<'a> {
    None(Ty<'a>),
    Option(Ty<'a>),
    Vec(Ty<'a>),
}

pub enum Ty<'a> {
    Literal(Lit),
    Custom(&'a syn::Ty),
}

#[derive(Clone, Copy)]
pub enum Lit {
    Bool,
    Char,
    Int(syn::IntTy),
    Str,
    ByteStr,
    Float(syn::FloatTy),
}

impl<'a> TryFrom<&'a syn::DeriveInput> for Struct<'a> {
    type Err = Error;

    fn try_from(ast: &'a syn::DeriveInput) -> Result<Self> {
        let fields = match ast.body {
            syn::Body::Struct(syn::VariantData::Struct(ref fields)) => fields,
            _ => bail!(ErrorKind::StructBody),
        };

        if !ast.generics.ty_params.is_empty() {
            bail!(ErrorKind::TyParams(ast.generics.ty_params.clone()));
        }

        if ast.generics.lifetimes.len() > 1 {
            bail!(ErrorKind::Lifetimes(ast.generics.lifetimes.clone()));
        }

        Ok(Struct {
            ast: ast,
            lifetime: ast.generics
                .lifetimes
                .iter()
                .next()
                .map(|l| &l.lifetime),
            fields: fields.iter().map(Field::try_from).collect::<Result<_>>()?,
        })
    }
}

impl<'a> TryFrom<&'a syn::Field> for Field<'a> {
    type Err = Error;

    fn try_from(ast: &'a syn::Field) -> Result<Self> {
        Ok(Field {
            ast: ast,
            ident: ast.ident.as_ref().unwrap(),
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
            _ => bail!(ErrorKind::Ty),
        };

        if path.global || path.segments.len() != 1 {
            bail!(ErrorKind::Ty);
        }

        let segment = &path.segments[0];
        Ok(match segment.ident.as_ref() {
            "Option" => Wrapper::Option((&segment.parameters).try_into()?),
            "Vec" => Wrapper::Vec((&segment.parameters).try_into()?),
            "bool" => Wrapper::None(Ty::Literal(Lit::Bool)),
            _ => bail!(ErrorKind::Ty),
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

impl<'a> TryFrom<&'a syn::PathParameters> for Ty<'a> {
    type Err = Error;

    fn try_from(p: &'a syn::PathParameters) -> Result<Self> {
        let data = match *p {
            syn::PathParameters::AngleBracketed(ref data) => data,
            syn::PathParameters::Parenthesized(_) => bail!(ErrorKind::Ty),
        };

        if !data.lifetimes.is_empty() || !data.bindings.is_empty() {
            bail!(ErrorKind::Ty);
        }

        if data.types.len() != 1 {
            bail!(ErrorKind::Ty);
        }

        (&data.types[0]).try_into()
    }
}

impl<'a> TryFrom<&'a syn::Ty> for Ty<'a> {
    type Err = Error;

    fn try_from(ty: &'a syn::Ty) -> Result<Self> {
        Ok(match *ty {
            syn::Ty::Path(None, ref path) => {
                if path.segments.is_empty() {
                    bail!(ErrorKind::Ty);
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
                    bail!(ErrorKind::Ty);
                }
                if ty.ty == syn::parse_type("str").unwrap() {
                    Ty::Literal(Lit::Str)
                } else if ty.ty == syn::parse_type("[u8]").unwrap() {
                    Ty::Literal(Lit::ByteStr)
                } else {
                    bail!(ErrorKind::Ty)
                }
            }
            _ => bail!(ErrorKind::Ty),
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
