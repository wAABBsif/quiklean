use quiklean::config;
use quiklean::trash;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = config::get_config_path()?;
    let config = config::read_config_file(&config_path)?;
    let config = config::Config::from_json(&config)?;
    println!("Successfully loaded config from '{}'", config_path.to_string_lossy());

    let trash_files = trash::get_trash_files_dir()?;
    println!("Successfully found trash files at '{}'", trash_files.to_string_lossy());

    let trash_info = trash::get_trash_info_dir()?;
    println!("Successfully found trash info at '{}'", trash_info.to_string_lossy());

    let trash_entries = trash::TrashEntry::read_trash_dir(&trash_files, &trash_info)?;
    for entry in trash_entries {
        let info = match trash::TrashInfo::read_file(&entry.info) {
            Ok(v) => v,
            Err(e) => {
                println!("Could not read info file '{}': ({e})", entry.info.to_string_lossy());
                continue
            }
        };

        if chrono::Local::now() - info.deletion_date > config.trash.time_limit {
            match quiklean::delete_file_or_directory(&entry.file) {
                Ok(_v) => println!("Successfully deleted '{}'", entry.file.to_string_lossy()),
                Err(e) => println!("Could not delete '{}' ({e})", entry.file.to_string_lossy()),
            };
            match std::fs::remove_file(&entry.info) {
                Ok(_v) => println!("Successfully deleted '{}'", entry.info.to_string_lossy()),
                Err(e) => println!("Could not delete '{}' ({e})", entry.info.to_string_lossy()),
            };
        }
    }
    Ok(()) 
}
