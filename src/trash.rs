use std::{fs, path::PathBuf};

pub fn get_trash_dir() -> Option<PathBuf> {
    let mut path = dirs::data_local_dir()?;
    path.push("Trash");
    if !fs::exists(&path).ok()? {
        return None
    }
    return Some(path);
}

pub fn get_trash_files_dir() -> Option<PathBuf> {
    let mut path = get_trash_dir()?;
    path.push("files");
    if !fs::exists(&path).ok()? {
        return None
    }
    return Some(path);
}

pub fn get_trash_info_dir() -> Option<PathBuf> {
    let mut path = get_trash_dir()?;
    path.push("info");
    if !fs::exists(&path).ok()? {
        return None
    }
    return Some(path);
}

pub struct TrashEntry {
    pub file: PathBuf,
    pub info: PathBuf,
}

impl TrashEntry {
    pub fn new (file: PathBuf, info: PathBuf) -> TrashEntry {
        TrashEntry {
            file,
            info,
        }
    }

    pub fn read_trash_dir(files: &PathBuf, info: &PathBuf) -> Result<Vec<TrashEntry>,
        Box<dyn std::error::Error>>
    {
        let mut result: Vec<TrashEntry> = Vec::new();

        for path in fs::read_dir(files)? {
            if path.is_err() {
                continue
            }
            let name = path.unwrap().file_name();
            let mut file = files.clone();
            let mut info = info.clone();

            file.push(&name);
            info.push(&name);
            info.add_extension("trashinfo");
            if !fs::exists(&info).unwrap_or(false) {
                continue
            }

            result.push(TrashEntry::new(file, info));
        }

        Ok(result)
    }
}
