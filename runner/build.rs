use std::path::PathBuf;
use std::process::{Command, Stdio, exit};
use std::{env, fs};

fn main() {
    let _ = fs::remove_file(env!("CARGO_MANIFEST_DIR").to_string() + "../image.iso");

    let _ = fs::remove_file(env!("CARGO_MANIFEST_DIR").to_string() + "../iso/kernel");

    let iso_dir = PathBuf::from("../iso");

    let status = Command::new("cargo")
        .args([
            "build",
            "-Z",
            "unstable-options",
            "--manifest-path",
            &(env!("CARGO_MANIFEST_DIR").to_string() + "/../kernel/Cargo.toml"),
            "--target",
            #[cfg(feature = "aarch")]
            "./aarch64-none-none.json",
            #[cfg(not(feature = "aarch"))]
            "./x86_64-none-none.json",
            "-Z",
            "build-std=core,alloc,compiler_builtins",
            "--target-dir",
            "./target",
            "--artifact-dir",
            "./",
            "-vv",
        ])
        .status()
        .unwrap()
        .code();
    if status.is_none() || status.unwrap() != 0 {
        panic!("cannot continue with failed kernel build");
    }

    let kernel_executable_file = PathBuf::from("./kernel").canonicalize().unwrap();

    let kernel_dest = iso_dir.join("kernel");
    fs::copy(&kernel_executable_file, &kernel_dest).unwrap_or_else(|_| {
        println!("cargo:error=failed to build");
        exit(0)
    });
    Command::new("xorriso")
        .args(dbg!([
            "-as",
            "mkisofs",
            "-R",
            "-r",
            "-e",
            "-J",
            "-b",
            "boot/limine-bios-cd.bin",
            "-no-emul-boot",
            "-boot-load-size",
            "4",
            "-boot-info-table",
            "-hfsplus",
            "-apm-block-size",
            "2048",
            "--efi-boot",
            "boot/limine-uefi-cd.bin",
            "-efi-boot-part",
            "--efi-boot-image",
            "--protective-msdos-label",
            iso_dir.canonicalize().unwrap().to_str().unwrap(),
            "-o",
            "../image.iso",
        ]))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap();
    println!("cargo:rustc-env=ISO=image.iso");

    println!(
        "cargo:rerun-if-changed={}",
        concat!(env!("CARGO_MANIFEST_DIR"), "../kernel")
    );

    Command::new("limine")
        .args(["../image.iso"])
        .status()
        .unwrap();

    Command::new("qemu-img")
        .args(["resize", "-f", "raw", "../image.iso", "2G"])
        .status()
        .unwrap();
}
