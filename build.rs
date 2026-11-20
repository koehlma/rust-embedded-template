//! Puts `memory.x` on the linker search path and selects the linker scripts.

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir =
        PathBuf::from(env::var_os("OUT_DIR").expect("Cargo sets `OUT_DIR` for build scripts"));
    fs::copy("memory.x", out_dir.join("memory.x")).expect("failed to copy `memory.x`");
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=memory.x");

    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
