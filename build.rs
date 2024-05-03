use std::env::var;
use cmake;

fn main() {
    if let Ok(_) = var("CARGO_FEATURE_SYCL") {
        const DPCPP_LINKER: &str = concat!(env!("DPCPP_HOME"), "/bin/clang++");
        const DPCPP_LIBS: &str = concat!(env!("DPCPP_HOME"), "/lib");
        
        let dst = cmake::Config::new("libsycl_solvers")
            .build();

        println!("cargo:rustc-link-search=native={}", dst.display());
        println!("cargo:rustc-link-search=native={DPCPP_LIBS}");
        println!("cargo:rustc-link-lib=static=sycl_solvers");
        println!("cargo:rustc-link-lib=dylib=sycl");
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-linker={DPCPP_LINKER}");
    }
}
