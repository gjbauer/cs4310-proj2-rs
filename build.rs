fn main() {
	bindgen::Builder::default()
		.header("src/disk.h")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.unwrap()
		.write_to_file(
		std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap())
		.join("disk.rs"),
        )
	.unwrap();
	println!("cargo:rerun-if-changed=src/disk.c");
	cc::Build::new().file("src/disk.c").compile("disk.a");
}
