fn main() {
    println!("cargo:rerun-if-changed=src/proto");
    // The daedalus `gen_simulate_data!` proc macro reads the CAN spec via
    // `fs::read_to_string` at expansion time, which cargo does not track on its
    // own. Rebuild when the spec changes so generated sim data stays fresh.
    println!("cargo:rerun-if-changed=Odyssey-Definitions");

    protobuf_codegen::Codegen::new()
        .pure()
        .includes(["src/proto"])
        .input("src/proto/serverdata.proto")
        .out_dir("src/proto")
        .run_from_script();
}
