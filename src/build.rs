fn main() {
    cc::Build::new()
        .file("directory.c")
        .compile("directory-def");
}
