use std::env;
use std::path::PathBuf;

fn main() {

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/wrapper.hpp");
    println!("cargo:rerun-if-changed=../src");
    
    let mut cmake_config = cmake::Config::new("../");

    // we need to keep the inline functions for gcc
    cmake_config.define("CMAKE_CXX_FLAGS", "-fkeep-inline-functions");

    let out_dir = std::env::var("OUT_DIR").unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir.clone() + "/lib");
    println!("cargo:rustc-link-lib=static=Foo");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", out_dir.clone() + "/include"))
        .clang_args(&["-x", "c++"])
        .generate_inline_functions(true)
        .header("wrapper.h")
        .opaque_type("std::.*")
        .allowlist_type("Foo")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}