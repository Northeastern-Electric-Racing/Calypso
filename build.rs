use std::process::Command;

/* Prebuild script */
fn main() {
    println!("cargo:rerun-if-env-changed=ALWAYS_RUN");

    // Create a new command that runs bash
    let mut command = Command::new("python3");

    // Pass the script name as an argument
    command.arg("./calypsogen.py");

    // Execute the command
    // This will download a file called ncbi_dataset.zip in the current directory
    command.output().expect("Failed to generate can messages!");
}