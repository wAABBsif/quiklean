use std::path::PathBuf;

use quiklean::config;
use quiklean::trash;
use quiklean::folder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = config::get_config_path()?;
    let config = config::read_config_file(&config_path)?;
    let config = config::Config::from_json(&config)?;
    println!("Loaded config from '{}'", config_path.to_string_lossy());

    match handle_trash(config.trash) {
        Ok(_v) => println!("Finished cleaning trash"),
        Err(e) => println!("Failed to clean trash: {e}")
    }
    if config.folders.is_some() {
        let folders = config.folders.unwrap();
        for folder in folders {
            match handle_folder(folder) {
                Ok(v) => println!("Finished cleaning folder ({})", v.to_string_lossy()),
                Err(e) => println!("Failed to clean folder: {e}"),
            }
        }
    }
    Ok(()) 
}

fn handle_trash(config: Option<config::Trash>) -> Result<(), Box<dyn std::error::Error>> {
    if config.is_none() {
        return Ok(())
    }
    let config = config.unwrap();
    let trash_files = trash::get_trash_files_dir()?;
    println!("Found trash files at '{}'", trash_files.to_string_lossy());

    let trash_info = trash::get_trash_info_dir()?;
    println!("Found trash info at '{}'", trash_info.to_string_lossy());

    let trash_entries = trash::TrashEntry::read_trash_dir(&trash_files, &trash_info)?;
    println!("Gathered all trash files");
    let time_limit = config.time_limit.try_into()?;
    let mut trash_cleaner = trash::TrashEntry::create_cleaner(trash_entries, time_limit);
    trash_cleaner.clean();
    Ok(())
}

fn handle_folder(config: config::Folder) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let target = folder::path_from_string(&config.target)?;
    let files = folder::read_folder(&target)?;
    let time_limit = config.time_limit.try_into()?;
    let mut cleaner = folder::create_cleaner(files, time_limit);
    cleaner.clean();
    Ok(target)
}
