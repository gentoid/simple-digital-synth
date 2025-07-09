use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=memory.x");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    fs::copy("memory.x", out.join("memory.x"))
        .expect("Could not copy memory.x to OUT_DIR");

    // println!("cargo:rustc-link-search={}", out.display());
}
