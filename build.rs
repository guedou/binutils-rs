// Guillaume Valadon <guillaume@valadon.net>
// binutils - build.rs

use std::env;
use std::ffi;
use std::fs::File;
use std::io::Read;
use std::path;
use std::process;

extern crate cc;

extern crate sha2;
use sha2::{Digest, Sha256};

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

fn execute_command_with_env(command: &str, args: Vec<&str>, env: &[(&str, &str)]) {
    let mut cmd = std::process::Command::new(command);
    cmd.args(&args);
    for (key, value) in env {
        cmd.env(key, value);
    }
    let status = cmd.status().expect(&format!("Failed to execute {}", command));
    if !status.success() {
        panic!("Error '{}' exited with code {:?}", command, status.code());
    }
}

fn change_dir(directory: &str) {
    // Go to another directory, and panic on error

    if !env::set_current_dir(directory).is_ok() {
        panic!(
            "\n\n  \
             Can't change dir to {}' !\n\n",
            directory
        );
    }
}

fn hash_file(filename: &str, hash_value: &str) -> bool {
    // Compute a SHA256 and return true if correct
    let mut f = File::open(filename).expect("file not found");
    let mut hasher = Sha256::new();

    loop {
        let mut buffer = [0; 256];

        let len = match f.read(&mut buffer[..]) {
            Err(_) => return false,
            Ok(len) => len,
        };
        if len == 0 {
            break;
        }

        hasher.input(&buffer[0..len]);
    }

    return format!("{:x}", hasher.result()) == hash_value;
}

fn build_binutils(version: &str, sha256sum: &str, output_directory: &str, targets: &str) {
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

    // Verify the SHA256 hash
    if !hash_file(&directory_filename, sha256sum) {
        panic!(
            "\n\n  \
             Incorrect hash value for {} !\n\n",
            filename
        );
    }

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

        // Set CFLAGS environment variable to include -fcommon
        // https://github.com/easybuilders/easybuild-easyconfigs/issues/11988
        std::env::set_var("CFLAGS", "-fcommon -g -O2");
    
        execute_command(
            "./configure",
            vec![&prefix_arg, &format!("--enable-targets={}", targets)],
        );
    
        // For make commands, we need to ensure CFLAGS is passed
        let make_env = vec![("CFLAGS", "-fcommon -g -O2")];
    
        execute_command_with_env("make", vec!["-j8"], &make_env);
        execute_command_with_env("make", vec!["install"], &make_env);
    
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
    let version = "2.43";
    let sha256 = "025c436d15049076ebe511d29651cc4785ee502965a8839936a65518582bdd64";

    // Retrieve targets to build
    let targets_var = match env::var_os("TARGETS") {
        Some(dir) => dir,
        None => ffi::OsString::from("all"),
    };
    let targets = targets_var
        .as_os_str()
        .to_str()
        .expect("Invalid TARGETS content!");

    // Get the current working directory
    let current_dir = env::current_dir().unwrap();

    // Where binutils will be built
    let out_directory = format!("{}/target", current_dir.to_str().unwrap());

    // Build binutils
    build_binutils(version, sha256, &out_directory, targets);

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

    // Link to zlib, zstd and sframe
    println!("cargo:rustc-link-search=native=/usr/lib/"); // Arch Linux
    println!("cargo:rustc-link-search=native=/usr/lib/{}-linux-gnu/", // Debian based
             env::var("CARGO_CFG_TARGET_ARCH").unwrap());
    println!("cargo:rustc-link-lib=static=z");
    println!("cargo:rustc-link-lib=sframe"); // Adjust if necessary
    println!("cargo:rustc-link-lib=zstd");

}
