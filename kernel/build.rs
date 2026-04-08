use std::path::PathBuf;

fn main() {
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let file = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join(format!("kernel-{}.ld", arch));
    let file = dbg!(file.to_str().unwrap());
    // Tell cargo to pass the linker script to the linker...
    println!("cargo:rustc-link-arg=-T{file}");
    // ...and to re-run if anything changes at all anywhere lol.
    println!("cargo:rerun-if-changed={}", env!("CARGO_MANIFEST_DIR"));
}
