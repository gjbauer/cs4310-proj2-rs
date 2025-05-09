fn main() {
    cc::Build::new()
        .file("src/c/pages.c")
        .compile("storage-mapping");
    cc::Build::new()
        .file("src/c/bitmap.c")
        .compile("bitmapping");
    cc::Build::new()
        .file("src/c/disk.c")
        .compile("i_o");
}
