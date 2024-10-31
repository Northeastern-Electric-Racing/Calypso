use std::process::Command;

/* Prebuild script */
fn main() {
    println!("cargo:rerun-if-changed=Embedded-Base");

    protobuf_codegen::Codegen::new()
        .pure()
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(["src/proto"])
        // Inputs must reside in some of include paths.
        .input("src/proto/command_data.proto")
        .input("src/proto/serverdata.proto")
        // Specify output directory relative to Cargo output directory.
        .out_dir("src")
        .run_from_script();
}
