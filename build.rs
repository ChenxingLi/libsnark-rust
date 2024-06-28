extern crate cmake;

use cmake::Config;

fn main() {
    // Run cmake and build the project
    let mut config = Config::new("libsnark");

    config.define("CMAKE_C_COMPILER", "/usr/bin/gcc-10");
    config.define("CMAKE_CXX_COMPILER", "/usr/bin/g++-10");
    if !cfg!(feature = "dynamic-code") {
        config.define("CURVE", "ALT_BN128");
    }
    config.define("WITH_PROCPS", "OFF");

    if cfg!(feature = "parallel") {
        config.define("MULTICORE", "ON");
    }

    let dst = config.build();

    // Tell cargo to look for libraries here
    println!(
        "cargo:rustc-link-search=native={}/build/libsnark",
        dst.clone().display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/depends/libff/libff",
        dst.clone().display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/depends",
        dst.clone().display()
    );

    let build_type = if std::env::var("PROFILE").unwrap() == "release" {
        ""
    } else {
        "d"
    };

    // Tell cargo to tell rustc to link the snark library
    if cfg!(feature = "dynamic-code") {
        println!("cargo:rustc-link-lib=static=zm{}", build_type);
    }
    println!("cargo:rustc-link-lib=static=ff{}", build_type);
    println!("cargo:rustc-link-lib=static=snark_demo{}", build_type);
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=gmp");
    if cfg!(feature = "parallel") {
        println!("cargo:rustc-link-lib=gomp");
    }

    println!("cargo:rerun-if-changed=libsnark");
}
