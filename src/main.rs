use std::path::PathBuf;

use quiklean::cleaning;
use quiklean::config;
use quiklean::trash;
use quiklean::folder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = config::get_config_path()?;
    let config = config::read_config_file(&config_path)?;
    println!("Located config path [{}]", config_path.to_string_lossy());
    let config = config::Config::from_json(&config)?;
    println!("Loaded config from [{}]", config_path.to_string_lossy());

    if let Err(e) = handle_trash(config.trash) {
        eprintln!("Error: failed to clean trash: ({e})");
    }
    if config.folders.is_some() {
        let folders = config.folders.unwrap();
        for folder in folders {
            let name = folder.target.clone();
            if let Err(e) = handle_folder(folder) {
                eprintln!("Error: failed to clean folder [{name}] ({e})");
            }
        }
    }
    Ok(()) 
}

fn handle_trash(config: Option<config::Trash>)
    -> Result<(), Box<dyn std::error::Error>> {
    if config.is_none() {
        return Ok(())
    }
    let config = config.unwrap();
    let trash_files = trash::get_trash_files_dir()?;
    println!("Found trash files at [{}]", trash_files.to_string_lossy());

    let trash_info = trash::get_trash_info_dir()?;
    println!("Found trash info at [{}]", trash_info.to_string_lossy());

    let trash_entries = trash::TrashEntry::read_trash_dir(&trash_files, &trash_info)?;
    println!("Gathered all trash files");
    let time_limit = config.time_limit.try_into()?;
    let cleaner_entries = trash::TrashEntry::create_cleaner_entries(trash_entries, time_limit);
    cleaning::clean(cleaner_entries, false);
    println!("Finished cleaning up trash");
    Ok(())
}

fn handle_folder(config: config::Folder)
    -> Result<PathBuf, Box<dyn std::error::Error>> {
    let target = folder::path_from_string(&config.target)?;
    let files = folder::read_folder(&target)?;
    let time_limit = config.time_limit.try_into()?;
    let directories = config.directories.unwrap_or_default();
    let cleaner_entries = folder::create_cleaner_entries(files, time_limit, directories);
    cleaning::clean(cleaner_entries, config.trash.unwrap_or_default());
    println!("Finished cleaning up folder [{}]", target.to_string_lossy());
    Ok(target)
}
