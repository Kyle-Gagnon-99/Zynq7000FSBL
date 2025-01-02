use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let out = env::var("OUT_DIR").unwrap();
    let out_dir = &PathBuf::from(&out);

    // Put the linker script somewhere the linker can find it
    File::create(out_dir.join("fsbl.x"))
        .unwrap()
        .write_all(include_bytes!("fsbl.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());

    println!("cargo:rerun-if-changed=fsbl.x");
}
