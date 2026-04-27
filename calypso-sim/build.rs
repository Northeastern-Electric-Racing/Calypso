fn main() {
    println!("cargo:rerun-if-changed=src/proto");

    protobuf_codegen::Codegen::new()
        .pure()
        .includes(["src/proto"])
        .input("src/proto/serverdata.proto")
        .out_dir("src/proto")
        .run_from_script();
}
