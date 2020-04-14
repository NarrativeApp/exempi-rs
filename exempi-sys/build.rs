extern crate pkg_config;
use std::env;
use std::path::Path;

fn main() {
    match pkg_config::Config::new()
        .atleast_version("2.4.0")
        .statik(true)
        .probe("exempi-2.0")
    {
        Ok(_) => (),
        Err(e) => {
            println!("cargo:warning=Exempi not found");
            panic!(e);
        }
    }

    if let Ok(libz_root) = env::var("DEP_Z_ROOT") {
        println!(
            "cargo:rustc-link-search={:?}",
            Path::new(&libz_root).join("build")
        );
    } else {
        println!("cargo:warning=Couldn't find zlib build root, assuming there's a built-in installation of zlib")
    }
    if let Ok(expat_root) = env::var("DEP_EXPAT_OUTDIR") {
        println!(
            "cargo:rustc-link-search={:?}",
            Path::new(&expat_root).join("lib")
        );
    } else {
        println!("cargo:warning=Couldn't find expat build root, assuming there's a built-in installation of expat")
    }

    println!("cargo:rustc-link-lib=static=expat");
    println!("cargo:rustc-link-lib=static=z");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "macos" {
        println!("cargo:rustc-link-lib=c++");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreServices");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
    }
}
