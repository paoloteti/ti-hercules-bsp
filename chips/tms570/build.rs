extern crate cc;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    cc::Build::new()
        .file("src/dabort.s")
        .compile("dabort");

    if target.contains("eabihf") {
        println!("cargo:rustc-cfg=vfp");
    }
}
