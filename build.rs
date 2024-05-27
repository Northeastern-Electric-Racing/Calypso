use std::process::Command;

/* Prebuild script */
fn main() {
    println!("cargo:rerun-if-changed=Embedded-Base");

    match Command::new("python3").arg("./calypsogen.py").status() {
        Ok(status) if status.success() => {
            println!("Python script executed successfully");
        },
        Ok(status) => {
            eprintln!("Python script exited with status: {}", status);
            std::process::exit(1);
        },
        Err(e) => {
            eprintln!("Failed to execute Python script: {}", e);
            std::process::exit(1);
        },
    }
}