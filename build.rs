fn main() {
    println!("cargo:rerun-if-changed=board.ld");
    println!("cargo:rerun-if-changed=tms570ls31xx.ld");
}
