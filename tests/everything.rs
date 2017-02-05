#[macro_use]
extern crate prom_attire;
#[macro_use]
extern crate quote;
extern crate syn;

use std::net::{IpAddr, Ipv4Addr};

#[test]
fn everything() {
    #[derive(PromAttire, PartialEq, Debug)]
    #[attire(scope = "everything", docs = "docs")]
    struct Everything<'a> {
        docs: Vec<&'a str>,
        literal_str: Option<&'a str>,
        literal_byte_str: Option<&'a [u8]>,
        literal_char: Option<char>,
        literal_u8: Option<u8>,
        literal_i8: Option<i8>,
        literal_u16: Option<u16>,
        literal_i16: Option<i16>,
        literal_u32: Option<u32>,
        literal_i32: Option<i32>,
        literal_u64: Option<u64>,
        literal_i64: Option<i64>,
        literal_usize: Option<usize>,
        literal_isize: Option<isize>,
        literal_f32: Option<f32>,
        literal_f64: Option<f64>,
        literal_bool: Option<bool>,
        parsed_byte_str: Option<&'a [u8]>,
        parsed_char: Option<char>,
        parsed_u8: Option<u8>,
        parsed_i8: Option<i8>,
        parsed_u16: Option<u16>,
        parsed_i16: Option<i16>,
        parsed_u32: Option<u32>,
        parsed_i32: Option<i32>,
        parsed_u64: Option<u64>,
        parsed_i64: Option<i64>,
        parsed_usize: Option<usize>,
        parsed_isize: Option<isize>,
        parsed_f32: Option<f32>,
        parsed_f64: Option<f64>,
        parsed_bool: Option<bool>,
        parsed_ip_addr: Option<IpAddr>,
        unwrapped_bool: bool,
        bool_word: Option<bool>,
    }

    let input = r#"
        #[everything(literal_str = "literal_str")]
        #[everything(literal_byte_str = b"literal_byte_str")]
        #[everything(literal_char = 'c')]
        #[everything(literal_u8 = 8u8)]
        #[everything(literal_i8 = 8i8)]
        #[everything(literal_u16 = 16u16)]
        #[everything(literal_i16 = 16i16)]
        #[everything(literal_u32 = 32u32)]
        #[everything(literal_i32 = 32i32)]
        #[everything(literal_u64 = 64u64)]
        #[everything(literal_i64 = 64i64)]
        #[everything(literal_usize = 128usize)]
        #[everything(literal_isize = 128isize)]
        #[everything(literal_f32 = 10.01f32)]
        #[everything(literal_f64 = 10.01f64)]
        #[everything(literal_bool = true)]
        #[everything(parsed_byte_str = "parsed_byte_str")]
        #[everything(parsed_char = "h")]
        #[everything(parsed_u8 = "9")]
        #[everything(parsed_i8 = "-7")]
        #[everything(parsed_u16 = "17")]
        #[everything(parsed_i16 = "-15")]
        #[everything(parsed_u32 = "33")]
        #[everything(parsed_i32 = "-31")]
        #[everything(parsed_u64 = "65")]
        #[everything(parsed_i64 = "-63")]
        #[everything(parsed_usize = "129")]
        #[everything(parsed_isize = "-127")]
        #[everything(parsed_f32 = "11.01")]
        #[everything(parsed_f64 = "10.02")]
        #[everything(parsed_bool = "true")]
        #[everything(parsed_ip_addr = "127.0.0.1")]
        #[everything(unwrapped_bool = true)]
        #[everything(bool_word)]
        #[doc = r" Some docs"]
        /// For this struct
        struct Foo {}
    "#;

    let ast = syn::parse_derive_input(input).unwrap();
    let attrs = Everything::from(ast.attrs.as_slice());

    let expected = Everything {
        docs: vec![" Some docs", " For this struct"],
        literal_str: Some("literal_str"),
        literal_byte_str: Some(b"literal_byte_str"),
        literal_char: Some('c'),
        literal_u8: Some(8),
        literal_i8: Some(8),
        literal_u16: Some(16),
        literal_i16: Some(16),
        literal_u32: Some(32),
        literal_i32: Some(32),
        literal_u64: Some(64),
        literal_i64: Some(64),
        literal_usize: Some(128),
        literal_isize: Some(128),
        literal_f32: Some(10.01),
        literal_f64: Some(10.01),
        literal_bool: Some(true),
        parsed_byte_str: Some(b"parsed_byte_str"),
        parsed_char: Some('h'),
        parsed_u8: Some(9),
        parsed_i8: Some(-7),
        parsed_u16: Some(17),
        parsed_i16: Some(-15),
        parsed_u32: Some(33),
        parsed_i32: Some(-31),
        parsed_u64: Some(65),
        parsed_i64: Some(-63),
        parsed_usize: Some(129),
        parsed_isize: Some(-127),
        parsed_f32: Some(11.01),
        parsed_f64: Some(10.02),
        parsed_bool: Some(true),
        parsed_ip_addr: Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
        unwrapped_bool: true,
        bool_word: Some(true),
    };

    assert_eq!(attrs, expected);
}
