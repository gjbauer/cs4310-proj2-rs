fn main() {
    bindgen::Builder::default()
        .header("src/c/pages.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap()
        .write_to_file(
            std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap())
                .join("src/pages.rs"),
        )
        .unwrap();
    println!("cargo:rerun-if-changed=src/c_code.c");
    cc::Build::new().file("src/c/pages.c").compile("src/c/pages.a");
}
