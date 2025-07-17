fn main() {
    cc::Build::new()
        .file("src/chapter_5/interfacing/multiply.c")
        .file("src/chapter_5/interfacing/dot_product.c")
        .compile("libmylib.a");
}
