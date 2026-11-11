pub struct Entry {
    pub path: std::path::PathBuf,
}

pub fn clean(entries: Vec<Entry>) {
    for entry in entries {
        match entry.delete() {
            Ok(_v) => println!("Deleted [{}]", entry.path.to_string_lossy()),
            Err(e) => eprintln!("Error: could not delete [{}]: {e}",
                entry.path.to_string_lossy()),
        };
    }
}

impl Entry {
    pub fn new(path: std::path::PathBuf) -> Entry {
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
