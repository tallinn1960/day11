fn main() {
    use cmake::Config;
    let dst = Config::new(".")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "YES")
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=day11cpp");
    println!("cargo:rerun-if-changed=day11cpp.cpp");
}

