use quiklean::config;
use quiklean::trash;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = config::get_config_path()?;
    let config = config::read_config_file(&config_path)?;
    let config = config::Config::from_json(&config)?;
    println!("Successfully loaded config from '{}'", config_path.to_string_lossy());

    match handle_trash(&config.trash) {
        Ok(v) => v,
        Err(_e) => println!("Failed to clean up trash")
    }
    Ok(()) 
}

fn handle_trash(config: &config::Trash) -> Result<(), Box<dyn std::error::Error>> {
    if !config.is_enabled {
        return Ok(())
    }
    let trash_files = trash::get_trash_files_dir()?;
    println!("Successfully found trash files at '{}'", trash_files.to_string_lossy());

    let trash_info = trash::get_trash_info_dir()?;
    println!("Successfully found trash info at '{}'", trash_info.to_string_lossy());

    let trash_entries = trash::TrashEntry::read_trash_dir(&trash_files, &trash_info)?;
    println!("Successfully gathered all trash files");
    let mut trash_cleaner = trash::TrashEntry::create_cleaner(trash_entries, config.time_limit);
    trash_cleaner.clean();
    Ok(())
}
