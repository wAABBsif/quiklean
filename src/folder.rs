use std::{fs, path::{Path, PathBuf}, str::FromStr};

use crate::{cleaning, config};

pub fn path_from_string(path: impl AsRef<str>) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path = String::from(path.as_ref());
    // Use the '~' as a shortcut for the home directory, as is common in Linux
    if path.contains("~") {
        let home = dirs::home_dir().ok_or_else(|| "could not find home directory")?;
        let home = home.to_string_lossy();
        path = path.replace("~", &home);
    }

    Ok(PathBuf::from_str(&path)?)
}

// Get the path of every file and directory in a directory
pub fn read_folder(dir: impl AsRef<Path>) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut result: Vec<PathBuf> = Vec::new();
    let dir = dir.as_ref();

    for path in fs::read_dir(dir)? {
        if path.is_err() {
            continue
        }

        let mut file = PathBuf::from(dir);
        let name = path.unwrap().file_name();
        file.push(name);
        result.push(file);
    }

    Ok(result)
}

// Create a vector of every single entry which needs to be deleted/trashed
pub fn create_cleaner_entries(entries: &[PathBuf], time_limit: chrono::TimeDelta,
    directories: config::Directories, time_metric: config::TimeMetric) -> Vec<cleaning::Entry>
{
    let mut result = vec![];
    let now = chrono::Local::now();
    for entry in entries { 
        // Determine behavior if entry is directory depending on the configuration
        if entry.is_dir() {
            match directories {
                config::Directories::Disabled => continue,
                config::Directories::Recursive => {
                    let dir_paths = match read_folder(entry) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("Could not read directory [{}] ({e})",
                                entry.to_string_lossy());
                            continue
                        }
                    };
                    let mut dir_vec = create_cleaner_entries(&dir_paths, time_limit,
                        directories, time_metric);
                    result.append(&mut dir_vec);
                    continue
                },
                config::Directories::Enabled => ()
            }
        }
        // Get the modified, accessed, or created time of the file, depending on config
        let metric_time = match get_metric_time(&entry, time_metric) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error: could not read File {time_metric} of [{}] ({e})",
                    entry.to_string_lossy());
                continue
            }
        };
        // Add file to list of files which need to be cleaned if the time metric is old enough
        if now - metric_time > time_limit {
            result.push(cleaning::Entry::new(entry));
        }
    }
    result
}

fn get_metric_time(path: impl AsRef<Path>, metric: config::TimeMetric)
    -> Result<chrono::DateTime<chrono::Local>, Box<dyn std::error::Error>>
{
    let metadata = std::fs::metadata(path)?;
    // We can read all of these time metrics from the metadata
    let metric_time = match metric {
        config::TimeMetric::Created => metadata.created(),
        config::TimeMetric::Modified => metadata.modified(),
        config::TimeMetric::Accessed => metadata.accessed()
    }?;
    let metric_time: chrono::DateTime<chrono::Local> = chrono::DateTime::from(metric_time);
    Ok(metric_time)
}
