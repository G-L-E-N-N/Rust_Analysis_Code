fn main() {
    cc::Build::new()
        .file("src/chapter_5/interfacing/multiply.c")
        .compile("libmultiply.a");
}
