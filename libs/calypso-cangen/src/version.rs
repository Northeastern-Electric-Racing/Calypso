use std::{fs, io};

use sha2::{Digest, Sha256};

use crate::CANGEN_SPEC_PATH;

pub fn version_canfiles() -> Result<String, Box<dyn std::error::Error>> {
    let mut hasher = Sha256::new();

    for __entry in fs::read_dir(CANGEN_SPEC_PATH)? {
        let __path = __entry?.path();
        if __path.is_file() && __path.extension().is_some_and(|ext| ext == "json") {
            let mut file = fs::File::open(__path).unwrap();

            let _ = io::copy(&mut file, &mut hasher).unwrap();
        }
    }
    let hash_bytes = hasher.finalize();
    Ok(format!("{:x}", hash_bytes))
}
