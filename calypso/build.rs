use calypso_cangen::validate::*;
use std::process;

/* Prebuild script */
fn main() {
    println!("cargo:rerun-if-changed=src/proto");

    protobuf_codegen::Codegen::new()
        .pure()
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(["../calypso/src/proto"])
        // Inputs must reside in some of include paths.
        .input("../calypso/src/proto/command_data.proto")
        .input("../calypso/src/proto/serverdata.proto")
        // Specify output directory relative to Cargo output directory.
        .out_dir("../calypso/src/proto")
        .run_from_script();

    // Validate CAN spec
    match validate_all_spec() {
        Ok(()) => {}
        Err(errors) => {
            for error in errors {
                // The \x1b[...m is an ANSI escape sequence for colored terminal output
                println!("\x1b[31;1mCAN spec error:\x1b[0m {}", error);
            }
            process::exit(1);
        }
    }
}
