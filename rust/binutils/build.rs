// build.rs

// Bring in a dependency on an externally maintained `cc` package which manages
// invoking the C compiler.
extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/utils.c")
        .include("/usr/")
        .compile("utils");

    // Locally compiled binutils libraries path
    println!("cargo:rustc-link-search={}", "../../binutils-2.29.1/built/lib/");
}
