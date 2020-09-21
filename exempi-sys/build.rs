extern crate pkg_config;

#[cfg(feature = "static")]
fn link_static() {
    use std::env;
    use std::path::Path;

    let libz_root = env::var("DEP_Z_ROOT")
        .expect("Couldn't find zlib build root, assuming there's a built-in installation of zlib");
    println!(
        "cargo:rustc-link-search={:?}",
        Path::new(&libz_root).join("build")
    );
    let expat_root = env::var("DEP_EXPAT_OUTDIR")
        .expect("Couldn't find expat build root, assuming there's a built-in installation of expat");
    println!(
        "cargo:rustc-link-search={:?}",
        Path::new(&expat_root).join("lib")
    );

    // println!("cargo:rustc-link-lib=static=expat");
    // println!("cargo:rustc-link-lib=static=z");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "macos" {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreServices");
    }
}

#[cfg(feature = "static")]
fn build_exempi() {
    use std::env;
    use std::path::Path;
    println!("Building exempi from source...");

    // TODO: build exempi
    // let libz_root = env::var("DEP_Z_ROOT")
    //     .expect("Couldn't find zlib build root");
    // let libz_lib = Path::new(&libz_root).join("build").to_str().unwrap().to_owned();
    // let libz_inc = Path::new(&libz_root).join("include").to_str().unwrap().to_owned();
    // let expat_root = env::var("DEP_EXPAT_OUTDIR")
    //     .expect("Couldn't find expat build root");
    // let expat_lib = Path::new(&expat_root).join("lib").to_str().unwrap().to_owned();
    // let expat_inc = Path::new(&expat_root).join("include").to_str().unwrap().to_owned();

    let dst = autotools::Config::new("src/exempi")
        .reconf("-ivf")
        // .cflag(format!("-L{} -I{}", libz_lib, libz_inc))
        // .cflag(format!("-L{} -I{}", expat_lib, expat_inc))
        // .cxxflag(format!("-L{} -I{}", libz_lib, libz_inc))
        // .cxxflag(format!("-L{} -I{}", expat_lib, expat_inc))
        // .ldflag(format!("-L{}", libz_lib))
        // .ldflag(format!("-L{}", expat_lib))
        .build();

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=exempi");
    println!("cargo:include={}", dst.join("include").display());
    println!("cargo:lib={}", dst.join("lib").display());
    println!("cargo:root={}", dst.display());
    // if target_os == "macos" {
    //     println!("cargo:rustc-link-lib=c++");
    // } else {
    //     println!("cargo:rustc-link-lib=stdc++");
    // }
}

fn main() {
    if cfg!(feature = "static") {
        #[cfg(feature = "static")]
        {
            build_exempi();
            //link_static();
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
