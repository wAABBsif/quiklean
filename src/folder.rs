use std::{fs, path::PathBuf, str::FromStr};

pub fn path_from_string(path: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path = String::from(path);
    if path.contains("~") {
        let home = dirs::home_dir().ok_or_else(|| "Could not find home directory")?;
        let home = home.to_string_lossy();
        path = path.replace("~", &home);
    }
    Ok(PathBuf::from_str(&path)?)
}

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
