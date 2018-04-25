// Guillaume Valadon <guillaume@valadon.net>
// binutils - build.rs

use std::env;
use std::path;
use std::process;

extern crate cc;


fn execute_command(command: &str, arguments: Vec<&str>) {
    // Execute a command, and panic on any error
     
    let status = process::Command::new(command).args(arguments).status();
    match status {
        Ok(exit) => match exit.success() {
            true => (),
            false => panic!(
                "\n\n  \
                 Error '{}' exited with code {}\n\n",
                command,
                exit.code().unwrap()
            ),
        },
        Err(e) => panic!(
            "\n\n  \
             Error with '{}': {}\n\n",
            command, e
        ),
    };
}


fn change_dir(directory: &str) {
    // Go to another directory, and panic on error

    if !env::set_current_dir(directory).is_ok() {
        panic!(
            "\n\n  \
             Can't change dir to ;{}' !\n\n",
            directory
        );
    }
}


fn build_binutils(version: &str) {
    // Build binutils

    let binutils_name = format!("binutils-{}", version);
    let filename = format!("{}.tar.gz", binutils_name);

    // Calls commands to build binutils
    if path::Path::new("built/").exists() {
        return;
    }

    // Check if the tarball exist
    if !path::Path::new(&filename).exists() {
        panic!(
            "\n\n  \
             Please download {} !\n    \
             ex: curl https://ftp.gnu.org/gnu/binutils/{} -O\n\n",
            filename, filename
        );
    }

    // Call tar
    if !path::Path::new(&binutils_name).exists() {
        execute_command("tar", vec!["xzf", &filename]);
    }

    // Calls commands to build binutils
    if path::Path::new(&binutils_name).exists() {

        let crate_dir = env::current_dir().unwrap();
        change_dir(&binutils_name);
        let prefix_arg = format!("--prefix={}/built/", crate_dir.display());
        execute_command(
            "./configure",
            vec![&prefix_arg, "--enable-shared", "--enable-targets=all"],
        );
        execute_command("make", vec!["-j8"]);
        execute_command("make", vec!["install"]);
        execute_command("cp", vec!["opcodes/config.h", "../built/include/"]);
        change_dir("..");
    }
}


fn main() {

    let version = "2.29.1";

    build_binutils(version);

    cc::Build::new()
        .file("src/utils.c")
        .include("built/include/")
        .compile("utils");

    // Locally compiled binutils libraries path
    println!("cargo:rustc-link-search=built/lib/");
}
