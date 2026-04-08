use std::process::Command;

#[cfg(not(feature = "aarch"))]
fn main() {
    Command::new("qemu-system-x86_64")
        .args([
            "-cdrom",
            env!("ISO"),
            "-m",
            "1G",
            "-accel",
            "tcg",
            "-drive",
            "if=pflash,unit=0,format=raw,file=runner/OVMF_X64.fd",
        ])
        .status()
        .unwrap();
}
