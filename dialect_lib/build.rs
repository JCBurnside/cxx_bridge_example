fn main() {
    let mut bridge = cxx_build::bridge("src/lib.rs")
        
        ;
    // bridge.compile("bridge");
    let cpp_lib = cmake::Config::new("../mlir_dialect_cpp_part")
        .init_cxx_cfg(bridge)
        
        .build();
    println!("cargo:rustc-link-search=native={}/lib", cpp_lib.display());
    println!("cargo:rustc-link-lib=static=foo_dialect");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
