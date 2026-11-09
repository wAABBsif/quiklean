use std::{fs, path::PathBuf};

pub fn read_folder(dir: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut result: Vec<PathBuf> = Vec::new();

    for path in fs::read_dir(dir)? {
        if path.is_err() {
            continue
        }

        let mut file = dir.clone();
        let name = path.unwrap().file_name();
        file.push(&name);
        result.push(file);
    }

    Ok(result)
}
