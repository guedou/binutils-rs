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

fn build_binutils(version: &str, output_directory: &str) {
    // Build binutils from source

    let binutils_name = format!("binutils-{}", version);
    let filename = format!("{}.tar.gz", binutils_name);
    let directory_filename = format!("{}/{}", output_directory, filename);

    // Check if binutils is already built
    if path::Path::new(&format!("{}/built/", output_directory)).exists() {
        return;
    }

    // Check if the tarball exists, or download it
    if !path::Path::new(&filename).exists() {
        execute_command(
            "curl",
            vec![
                format!("https://ftp.gnu.org/gnu/binutils/{}", filename).as_str(),
                "--output",
                &directory_filename,
            ],
        );
    }

    // GV: verify checksum

    // Check if the tarball exists after calling curl
    if !path::Path::new(&directory_filename).exists() {
        panic!(
            "\n\n  \
             Can't download {} to {} using curl!\n\n",
            filename, directory_filename
        );
    }

    // Call tar
    change_dir(output_directory);
    if !path::Path::new(&binutils_name).exists() {
        execute_command("tar", vec!["xzf", &filename]);
    }

    // Calls commands to build binutils
    if path::Path::new(&binutils_name).exists() {
        change_dir(&binutils_name);
        let prefix_arg = format!("--prefix={}/built/", output_directory);
        execute_command("./configure", vec![&prefix_arg, "--enable-targets=all"]);
        execute_command("make", vec!["-j8"]);
        execute_command("make", vec!["install"]);

        // Copy useful files
        execute_command(
            "cp",
            vec![
                "opcodes/config.h",
                &format!("{}/built/include/", output_directory),
            ],
        );
        execute_command(
            "cp",
            vec![
                "libiberty/libiberty.a",
                &format!("{}/built/lib/", output_directory),
            ],
        );
    }
}

fn main() {
    //let version = ("2.29.1", "0d9d2bbf71e17903f26a676e7fba7c200e581c84b8f2f43e72d875d0e638771c");
    let version = "2.29.1";

    // Extract the out directory from the env variable
    let out_dir = match env::var_os("OUT_DIR") {
        Some(dir) => dir,
        None => panic!(
            "\n\n  \
             OUT_DIR variable is not set!\n\n"
        ),
    };

    // Build binutils
    let current_dir = env::current_dir().unwrap();
    let out_directory = out_dir
        .as_os_str()
        .to_str()
        .expect("Invalid OUT_DIR content!");
    build_binutils(version, out_directory);

    // Build our C helpers
    change_dir(current_dir.to_str().unwrap());
    cc::Build::new()
        .file("src/helpers.c")
        .include(format!("{}/built/include/", out_directory))
        .compile("helpers");

    // Locally compiled binutils libraries path
    println!(
        "cargo:rustc-link-search=native={}",
        format!("{}/built/lib/", out_directory)
    );
    println!("cargo:rustc-link-lib=static=bfd");
    println!("cargo:rustc-link-lib=static=opcodes");
    println!("cargo:rustc-link-lib=static=iberty");

    // Link to zlib
    println!("cargo:rustc-link-search=native=/usr/lib/");
    println!("cargo:rustc-link-lib=static=z");
}
