use syn;

error_chain! {
    errors {
        StructBody {
            description("#[derive(FromAttributes)] can only be applied to normal structs")
        }

        TyParams(ty_params: Vec<syn::TyParam>) {
            description("#[derive(FromAttributes)] cannot have type parameters on the struct")
        }

        Lifetimes(lifetimes: Vec<syn::LifetimeDef>) {
            description("#[derive(FromAttributes)] can have a maximum of 1 lifetime on the struct")
        }
    }
}
