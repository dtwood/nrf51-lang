fn main() {
    println!("cargo:rustc-link-lib=nosys");
    println!("cargo:rustc-link-lib=c");
    println!("cargo:rustc-link-lib=gcc");
    println!("cargo:rustc-link-lib=c");
}
