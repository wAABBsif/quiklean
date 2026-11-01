use quiklean::config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = config::get_config_path().ok_or_else(|| "Expected to find config path")?;
    let config = config::Config::from_json(&config::read_config_file(&config_path)?)?;
    println!("Successfully loaded config file {}", config_path.to_str().unwrap_or("UNKNOWN"));
    println!("Time limit is {} seconds", config.time_limit.num_seconds());

    Ok(()) 
}
