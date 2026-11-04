use quiklean::config;
use quiklean::trash;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = config::get_config_path().ok_or_else(||
        "Expected to find config path")?;
    let config = config::Config::from_json(&config::read_config_file(&config_path)?)?;
    println!("Successfully loaded config file {}", config_path.to_str().unwrap_or("UNKNOWN"));
    println!("Time limit is {} seconds", config.time_limit.num_seconds());

    let trash_files = trash::get_trash_files_dir().ok_or_else(
        || "Expected to find trash files path")?;
    let trash_info = trash::get_trash_info_dir().ok_or_else(
        || "Expected to find trash info path")?;
    println!("Successfully found trash files at {}", trash_files.to_str().unwrap_or("UNKNOWN"));
    println!("Successfully found trash info at {}", trash_info.to_str().unwrap_or("UNKNOWN"));
    let trash_entries = trash::TrashEntry::read_trash_dir(&trash_files, &trash_info)?;
    for entry in trash_entries {
        let file = entry.file.to_str().unwrap_or("UNKNOWN");
        let info = entry.info.to_str().unwrap_or("UNKNOWN");
        println!("file: {file}, info: {info}");
    }
    Ok(()) 
}
