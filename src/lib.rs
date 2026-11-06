pub mod config;
pub mod trash;

pub fn delete_file_or_directory(path: &std::path::PathBuf) -> Result<(), std::io::Error> {
    if path.is_file() {
        std::fs::remove_file(path)?;
    } else {
        std::fs::remove_dir_all(path)?;
    }
    Ok(())
}
