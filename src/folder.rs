use std::{fs, path::PathBuf, str::FromStr};

use crate::{cleaning, config};

pub fn path_from_string(path: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path = String::from(path);
    if path.contains("~") {
        let home = dirs::home_dir().ok_or_else(|| "could not find home directory")?;
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

pub fn create_cleaner_entries(entries: Vec<PathBuf>, time_limit: chrono::TimeDelta,
    directories: config::Directories) -> Vec<cleaning::Entry>
{
    let mut result = vec![];
    let now = chrono::Local::now();
    for entry in entries { 
        if entry.is_dir() {
            match directories {
                config::Directories::Disabled => continue,
                config::Directories::Recursive => {
                    let dir_paths = match read_folder(&entry) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("Could not read directory [{}] ({e})",
                                entry.to_string_lossy());
                            continue
                        }
                    };
                    let mut dir_vec = create_cleaner_entries(dir_paths, time_limit, 
                        config::Directories::Recursive);
                    result.append(&mut dir_vec);
                    continue
                },
                config::Directories::Enabled => ()
            }
        }
        let modified_time = match get_modified_time(&entry) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error: could not read file modified of [{}] ({e})",
                    entry.to_string_lossy());
                continue
            }
        };
        if now - modified_time > time_limit {
            result.push(cleaning::Entry::new(entry));
        }
    }
    result
}

fn get_modified_time(path: &PathBuf)
    -> Result<chrono::DateTime<chrono::Local>, Box<dyn std::error::Error>>
{
    let metadata = std::fs::metadata(path)?;
    let modified_time = metadata.modified()?;
    let modified_time: chrono::DateTime<chrono::Local> = chrono::DateTime::from(modified_time);
    Ok(modified_time)
}
