extern crate rustc_version;

use std::error::Error;

use rustc_version::{version, Version};

fn main() {
    fn main() -> Result<(), Box<Error>> {
        if version()? >= Version::parse("1.26.0-nightly")? {
            println!("cargo:rustc-cfg=never_type");
        }

        Ok(())
    }
    main().unwrap()
}
