extern crate pkg_config;

#[cfg(feature = "static")]
fn link_static() {
    use std::env;
    println!("cargo:rustc-link-lib=dylib=expat");
    println!("cargo:rustc-link-lib=dylib=z");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "macos" {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreServices");
    }
}

#[cfg(feature = "static")]
fn build_exempi() {
    println!("Building exempi from source...");

    let dst = autotools::Config::new("src/exempi")
        .reconf("-ivf")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=exempi");
    println!("cargo:include={}", dst.join("include").display());
    println!("cargo:lib={}", dst.join("lib").display());
    println!("cargo:root={}", dst.display());

    // rebuild this library if any key files change (TODO: include more files)
    println!("cargo:rerun-if-changed=src/exempi/exempi/xmp.h");
}

fn main() {
    if cfg!(feature = "static") {
        // build exempi from source and link statically
        #[cfg(feature = "static")]
        {
            build_exempi();
            link_static();
        }
    } else {
        // attempt to link exempi dynamically
        match pkg_config::Config::new()
            .atleast_version("2.4.0")
            .probe("exempi-2.0")
        {
            Ok(_) => (),
            Err(e) => {
                println!("cargo:warning=Exempi not found");
                panic!(e);
            }
        }
    }
}
