pub struct Cleaner {
    pub entries: Vec<CleanerEntry>,
}

pub struct CleanerEntry {
    pub path: std::path::PathBuf,
}

impl Cleaner {
    pub fn new() -> Cleaner {
        Cleaner {
            entries: Vec::new()
        }
    }

    pub fn clean(&mut self) {
        for entry in &self.entries {
            match entry.delete() {
                Ok(_v) => println!("Deleted [{}]", entry.path.to_string_lossy()),
                Err(e) => eprintln!("Error: could not delete [{}]: {e}",
                    entry.path.to_string_lossy()),
            };
        }
    }
}

impl CleanerEntry {
    pub fn new(path: std::path::PathBuf) -> CleanerEntry {
        CleanerEntry {
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
