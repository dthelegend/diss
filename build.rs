fn main() {
    // TODO make this compile correctly without cuda as well as a build option
    cc::Build::new()
        .cuda(true)
        .cudart("static")
        .file("src/problem/qubo/solver/pes.backend.cu")
        .emit_rerun_if_env_changed(true)
        .compile("pes");
}
