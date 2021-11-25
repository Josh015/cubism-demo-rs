use std::{fs::File, path::PathBuf};

use ron::de::from_reader;
use serde::de::DeserializeOwned;

pub fn load_from_file<T: DeserializeOwned>(path: &str) -> T {
    let input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path);
    let f = File::open(&input_path)
        .expect(&format!("Failed opening config file: {:#?}", path)[..]);

    match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);

            std::process::exit(1);
        },
    }
}
