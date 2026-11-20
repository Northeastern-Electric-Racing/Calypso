fn main() {
    // Build scripts run from this crate's dir, so reach up to the root crate's
    // proto rather than keeping a copy (or a symlink, which Windows checks out
    // as a plain text file).
    println!("cargo:rerun-if-changed=../src/proto/serverdata.proto");
    // The daedalus `gen_simulate_data!` proc macro reads the CAN spec via
    // `fs::read_to_string` at expansion time, which cargo does not track on its
    // own. Rebuild when the spec changes so generated sim data stays fresh.
    println!("cargo:rerun-if-changed=../Odyssey-Definitions/can-messages");

    // Generated output only (git-ignored), so the dir may not exist on a fresh clone.
    std::fs::create_dir_all("src/proto").expect("create src/proto");

    protobuf_codegen::Codegen::new()
        .pure()
        .includes(["../src/proto"])
        .input("../src/proto/serverdata.proto")
        .out_dir("src/proto")
        .run_from_script();
}
