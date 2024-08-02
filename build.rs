use std::process::Command;

/* Prebuild script */
fn main() {
    println!("cargo:rerun-if-changed=Embedded-Base");

    match Command::new("python3").arg("./calypsogen.py").status() {
        Ok(status) if status.success() => {
            println!("Python script executed successfully");
        }
        Ok(status) => {
            eprintln!("Python script exited with status: {}", status);
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Failed to execute Python script: {}", e);
            std::process::exit(1);
        }
    }

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
