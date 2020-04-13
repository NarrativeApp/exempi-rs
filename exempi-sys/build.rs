extern crate pkg_config;
use std::env;

fn main() {
    match pkg_config::Config::new().atleast_version("2.4.0").statik(true).probe("exempi-2.0") {
        Ok(_) => (),
        Err(e) => {
            println!("cargo:warning=Exempi not found");
            panic!(e);
        }
    }
    println!("cargo:rustc-link-lib=expat");
    println!("cargo:rustc-link-lib=zlib");
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "macos" {
        println!("cargo:rustc-link-lib=c++");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreServices");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
    }
}
