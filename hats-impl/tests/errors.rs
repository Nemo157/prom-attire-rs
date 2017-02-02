extern crate hats_impl;

#[macro_use]
extern crate quote;
extern crate syn;

use hats_impl::{ Config, ErrorKind };

macro_rules! assert_error_kind {
    ($err:expr, $kind:pat) => {{
        let err = $err;
        match err.kind() {
            &$kind => (),
            _ => {
                panic!(
                    "expected error of kind {}, got: {:?}",
                    stringify!($kind),
                    err)
            }
        }
    }}
}

#[test]
fn enuum() {
    let input = quote! { enum A {} };
    let config = Config { scope: None };
    let result = hats_impl::derive(input.as_str(), config);
    assert_error_kind!(result.unwrap_err(), ErrorKind::StructBody)
}

#[test]
fn tuple_struct() {
    let input = quote! { struct A(); };
    let config = Config { scope: None };
    let result = hats_impl::derive(input.as_str(), config);
    assert_error_kind!(result.unwrap_err(), ErrorKind::StructBody)
}
