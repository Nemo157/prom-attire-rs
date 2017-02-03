//! Takes an input struct and extracts all the details necessary to generate
//! the From<&[Attribute]> implementation

use syn::{self, DeriveInput, Body, VariantData, Lifetime};

use {Config, ErrorKind, Result};

pub struct Struct<'a> {
    pub ast: &'a DeriveInput,
    pub lifetime: Option<&'a Lifetime>,
    pub fields: Vec<Field<'a>>,
}

pub struct Field<'a> {
    pub field: &'a syn::Field,
}

fn dissect_field<'a>(field: &'a syn::Field) -> Result<Field<'a>> {
    Ok(Field { field: field })
}

pub fn dissect<'a>(
    ast: &'a DeriveInput,
    config: &'a Config<'a>
) -> Result<Struct<'a>> {
    let fields = match ast.body {
        Body::Struct(VariantData::Struct(ref fields)) => fields,
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
        lifetime: ast.generics.lifetimes.iter().next().map(|l| &l.lifetime),
        fields: fields.iter().map(dissect_field).collect::<Result<_>>()?,
    })
}
