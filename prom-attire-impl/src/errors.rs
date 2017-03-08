use std::fmt;

use syn;
use quote;

struct Q<T: quote::ToTokens>(T);
impl<T: quote::ToTokens> fmt::Display for Q<T> {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        let t = &self.0;
        quote!(#t).fmt(w)
    }
}

error_chain! {
    errors {
        StructBody {
            description("can only be applied to normal structs")
        }

        TyParams(ty_params: Vec<syn::TyParam>) {
            description("cannot have type parameters on the struct")
        }

        Lifetimes(lifetimes: Vec<syn::LifetimeDef>) {
            description("can have a maximum of 1 lifetime on the struct")
            display("can have a maximum of 1 lifetime on the struct, found `{}`", lifetimes.len())
        }

        SplitFieldTys(split: String, ty: syn::Ty, field: syn::Field) {
            description("split fields must have same type")
            display(
                "field `{}` has type `{}`, but previous split_attribute_of(`{}`) fields had type `{}`",
                field.ident.as_ref().unwrap(),
                Q(&field.ty),
                split,
                Q(&ty))
        }

        Field(field: syn::Field) {
            description("field had an error")
            display("field `{}` had an error", field.ident.as_ref().unwrap())
        }

        WordValueNoDefault {
            description("if a `flag_value` is specified a `default` must also be specified")
        }

        Ty(ty: syn::Ty) {
            description("unsupported type")
            display("type `{}` is not supported", Q(&ty))
        }

        TyWrapperOrDefault(ty: syn::Ty) {
            description("unsupported unwrapped type")
            display("unwrapped type `{}` without a default is not supported, it must be enclosed in a `Vec` or `Option` or have a `default` specified", Q(&ty))
        }

        TyWrapperBad(ty: syn::Ty) {
            description("invalid Option/Vec type wrapper")
            display("type `{}` is not supported, it appears to be a `Vec` or `Option` but could not be parsed", Q(&ty))
        }

        TyRef(ty: syn::Ty) {
            description("unsupported reference type")
            display("type `{}` is not supported, only immutable `str` and `[u8]` reference types are supported", Q(&ty))
        }

        DocsTy(field: syn::Field) {
            description("docs field must be a Vec<&str>")
            display("docs field `{}` must be a Vec<&str>", field.ident.as_ref().unwrap())
        }
    }
}
