fn main() {
    // TODO make this compile correctly without cuda as well as a build option
    cc::Build::new()
        .cuda(true)
        .cudart("static")
        .include("src/problem/qubo/solver/")
        .file("src/problem/qubo/solver/mopso.backend.cu")
        .file("src/problem/qubo/solver/pes.backend.cu")
        .emit_rerun_if_env_changed(true)
        .compile("cuda_backends");
    println!("cargo:rerun-if-changed=src/**/*.h");
    println!("cargo:rerun-if-changed=src/**/*.cu");
}
