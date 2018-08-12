fn main() {
    println!("cargo:rerun-if-changed=bsp.ld");
    println!("cargo:rerun-if-changed=tms570ls31xx.ld");
}
