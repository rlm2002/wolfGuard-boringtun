fn main() {
    println!("cargo:rerun-if-env-changed=WOLFSSL_LIB_DIR");
    if let Ok(lib_dir) = std::env::var("WOLFSSL_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", lib_dir);
    }
}
