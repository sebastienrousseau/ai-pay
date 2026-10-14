//! Build script: rebuild when the binary source or this script changes.

fn main() {
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=build.rs");
}
