use crate::trash;
use std::path::PathBuf;

pub struct Entry {
    pub path: PathBuf,
}

pub fn clean(entries: Vec<Entry>, recycle: bool) {
    for entry in entries {
        if !recycle {
            match entry.delete() {
                Ok(_v) => println!("Deleted [{}]", entry.path.to_string_lossy()),
                Err(e) => eprintln!("Error: could not delete [{}]: {e}",
                    entry.path.to_string_lossy()),
            };
        } else {
            match trash::trash_file_or_directory(&entry.path) {
                Ok(_v) => println!("Trashed [{}]", entry.path.to_string_lossy()),
                Err(e) => eprintln!("Error: could not trash [{}]: {e}",
                    entry.path.to_string_lossy()),
            };
        }
    }
}

impl Entry {
    pub fn new(path: PathBuf) -> Entry {
        Entry {
            path: path,
        }
    }

    pub fn delete(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.path.is_file() {
            std::fs::remove_file(&self.path)?;
        } else {
            std::fs::remove_dir_all(&self.path)?;
        }
        Ok(())
    }
}
