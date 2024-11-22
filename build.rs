fn main() {
    println!("cargo:rerun-if-changed=memory.ld");
    println!("cargo:rustc-link-arg=-Tmemory.ld");
}
