use syn;

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
        }

        Field(field: syn::Field) {
            description("field had an error")
        }

        Ty {
            description("unsupported type")
        }

        DocsTy(field: syn::Field) {
            description("docs field must be a Vec<&str>")
        }
    }
}
