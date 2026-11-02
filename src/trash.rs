use std::{fs, path::PathBuf};

pub fn get_trash_dir() -> Option<PathBuf> {
    let mut path = dirs::data_local_dir()?;
    path.push("Trash");
    if !fs::exists(&path).ok()? {
        return None;
    }
    return Some(path);
}

pub fn get_trash_files_dir() -> Option<PathBuf> {
    let mut path = get_trash_dir()?;
    path.push("files");
    if !fs::exists(&path).ok()? {
        return None;
    }
    return Some(path);
}

pub fn get_trash_info_dir() -> Option<PathBuf> {
    let mut path = get_trash_dir()?;
    path.push("info");
    if !fs::exists(&path).ok()? {
        return None;
    }
    return Some(path);
}
