#![allow(missing_docs)]

#[cfg(feature = "Swift")]
fn main() {
    use cmake::Config;
    use std::env;
    use std::path::PathBuf;
    let dst = Config::new(".")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "YES")
        .generator("Ninja Multi-Config")
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=day11Swift");
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(env::var("OUT_DIR").unwrap().to_string() + "/day11.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    println!("cargo:rerun-if-changed=Swift/Sources/Day11/Day11.swift")
}

#[cfg(not(feature = "Swift"))]
fn main() {}
