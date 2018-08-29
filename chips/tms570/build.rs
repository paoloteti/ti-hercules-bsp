extern crate cc;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    let mut compiler = cc::Build::new();

    if target.contains("eabihf") {
        println!("cargo:rustc-cfg=vfp");
        compiler.define("INIT_VFP_REGISTER", "Y");
    }
    compiler.file("src/dabort.s")
            .file("src/syscore.s")
            .compile("core");
}
