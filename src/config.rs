use std::{fs, io::{Read, Write}, path::PathBuf};

#[derive(PartialEq, Eq, Debug)]
pub struct Config {
    pub trash: Trash
}

#[derive(PartialEq, Eq, Debug)]
pub struct Trash {
    pub time_limit: chrono::TimeDelta
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ConfigParser {
    trash: TrashParser
}

#[derive(serde::Serialize, serde::Deserialize)]
struct TrashParser {
    time_limit: TimeDeltaParser
}

#[derive(serde::Serialize, serde::Deserialize)]
struct TimeDeltaParser {
    weeks: i64,
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64
}

impl Config {
    pub fn default() -> Config {
        Config {
            trash: Trash {
                time_limit: chrono::TimeDelta::weeks(2)
            }
        }
    }

    pub fn from_json(json: &str) -> Result<Config, Box<dyn std::error::Error>>{
        let result: ConfigParser = serde_json::from_str(json)?;
        let result: Config = result.to_config()?;
        Ok(result)
    }

    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        let result = ConfigParser::from_config(self)?;
        let result = serde_json::to_string_pretty(&result)?;
        Ok(result)
    }
}

impl ConfigParser {
    fn from_config(config: &Config) -> Result<ConfigParser, Box<dyn std::error::Error>> {
        Ok(ConfigParser {
            trash: TrashParser::from_config(&config.trash)?
        })
    }

    fn to_config(&self) -> Result<Config, Box<dyn std::error::Error>> {
        Ok(Config {
            trash: self.trash.to_config()?
        })
    }
}

impl TrashParser {
    fn from_config(trash: &Trash) -> Result<TrashParser, Box<dyn std::error::Error>> {
        Ok(TrashParser {
            time_limit: TimeDeltaParser::from_config(&trash.time_limit)
        })
    }

    fn to_config(&self) -> Result<Trash, Box<dyn std::error::Error>> {
        Ok(Trash {
            time_limit: self.time_limit.to_config()?
        })
    }
}

impl TimeDeltaParser {
    fn from_config(delta: &chrono::TimeDelta) -> TimeDeltaParser {
        TimeDeltaParser {
            weeks: delta.num_weeks(),
            days: delta.num_days() % 7,
            hours: delta.num_hours() % 24,
            minutes: delta.num_minutes() % 60,
            seconds: delta.num_seconds() % 60
        }
    }

    fn to_config(&self) -> Result<chrono::TimeDelta, Box<dyn std::error::Error>> {
        Ok(
            chrono::TimeDelta::try_weeks(self.weeks).ok_or_else(|| "Too many weeks")?
            + chrono::TimeDelta::try_days(self.days).ok_or_else(|| "Too many days")?
            + chrono::TimeDelta::try_hours(self.hours).ok_or_else(|| "Too many hours")?
            + chrono::TimeDelta::try_minutes(self.minutes).ok_or_else(|| "Too many minutes")?
            + chrono::TimeDelta::try_seconds(self.seconds).ok_or_else(|| "Too many seconds")?
        )
    }
}

//Should be /home/[USER]/.config/quiklean/config.json
pub fn get_config_path() -> Result<PathBuf, Box <dyn std::error::Error>> {
    let mut config_dir = dirs::config_dir().ok_or_else(|| "Could not find config path")?;

    config_dir.push("quiklean/");
    fs::create_dir_all(&config_dir)?;

    config_dir.push("config.json");
    Ok(config_dir)
}

pub fn read_config_file(path: &PathBuf) -> Result<String, Box <dyn std::error::Error>> {
    let mut file: fs::File;

    if !fs::exists(&path)? {
        file = fs::File::create(&path)?;
        let contents = Config::to_json(&Config::default())?;
        write!(file, "{}", contents)?;
    }

    file = fs::File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    Ok(contents)
}

//These tests ensure that configurations can be converted to and from .json files.
#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_JSON: &str =
r#"{
    "trash": {
        "time_limit": {
            "weeks": 2,
            "days": 0,
            "hours": 0,
            "minutes": 0,
            "seconds": 0
        }
    }
}"#;


    #[test]
    fn test_inverse_conversion() {
        let default = Config::from_json(DEFAULT_JSON).unwrap();
        assert_eq!(Config::from_json(&default.to_json().unwrap()[..]).unwrap(), default);
    }

    #[test]
    fn test_default_matches_string() {
        assert_eq!(Config::default(), Config::from_json(DEFAULT_JSON).unwrap());
    }
}

