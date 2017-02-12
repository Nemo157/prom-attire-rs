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
            display("can have a maximum of 1 lifetime on the struct, found {}", lifetimes.len())
        }

        Field(field: syn::Field) {
            description("field had an error")
            display("field '{}' had an error", field.ident.as_ref().unwrap())
        }

        Ty {
            description("unsupported type")
        }

        DocsTy(field: syn::Field) {
            description("docs field must be a Vec<&str>")
            display("docs field '{}' must be a Vec<&str>", field.ident.as_ref().unwrap())
        }
    }
}
