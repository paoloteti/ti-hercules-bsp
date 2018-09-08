extern crate cc;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    cc::Build::new()
        .file("src/dabort.s")
        .file("src/cpustack.s")
        .compile("tms570");

    if target.contains("eabihf") {
        println!("cargo:rustc-cfg=vfp");
    }
}
