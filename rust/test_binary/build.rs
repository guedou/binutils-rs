// build.rs

// Bring in a dependency on an externally maintained `cc` package which manages
// invoking the C compiler.
extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/utils.c")
        .include("/usr/")
        .compile("utils");
}
