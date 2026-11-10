use std::{fs, io::{Read, Write}, path::PathBuf};

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
pub struct Config {
    pub trash: Option<Trash>,
    pub folders: Option<Vec<Folder>>
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
pub struct Trash {
    pub time_limit: TimeLimitConfig
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
pub struct Folder {
    pub target: String,
    pub time_limit: TimeLimitConfig
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
pub struct TimeLimitConfig {
    weeks: Option<i64>,
    days: Option<i64>,
    hours: Option<i64>,
    minutes: Option<i64>,
    seconds: Option<i64>
}

//Should be /home/[USER]/.config/quiklean/config.json
pub fn get_config_path() -> Result<PathBuf, Box <dyn std::error::Error>> {
    let mut config_dir = dirs::config_dir().ok_or_else(|| "could not find config path")?;

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

impl Config {
    pub fn default() -> Config {
        Config {
            trash: Some(Trash {
                time_limit: TimeLimitConfig::from(chrono::TimeDelta::weeks(2))
            }),
            folders: Some(vec![])
        }
    }

    pub fn from_json(json: &str) -> Result<Config, Box<dyn std::error::Error>>{
        Ok(serde_json::from_str(json)?)
    }

    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        let result = serde_json::to_string_pretty(&self)?;
        Ok(result)
    }
}

impl From<chrono::TimeDelta> for TimeLimitConfig {
    fn from(delta: chrono::TimeDelta) -> TimeLimitConfig {
        TimeLimitConfig {
            weeks: Some(delta.num_weeks()),
            days: Some(delta.num_days() % 7),
            hours: Some(delta.num_hours() % 24),
            minutes: Some(delta.num_minutes() % 60),
            seconds: Some(delta.num_seconds() % 60)
        }
    }
}

impl TryInto<chrono::TimeDelta> for TimeLimitConfig {
    type Error = String;
    fn try_into(self) -> Result<chrono::TimeDelta, String> {
        let error_msg = "time limit is too high";
        Ok(chrono::TimeDelta::try_weeks(self.weeks.unwrap_or_default()).ok_or_else(
                || error_msg)?
            + chrono::TimeDelta::try_days(self.days.unwrap_or_default()).ok_or_else(
                || error_msg)?
            + chrono::TimeDelta::try_hours(self.hours.unwrap_or_default()).ok_or_else(
                || error_msg)?
            + chrono::TimeDelta::try_minutes(self.minutes.unwrap_or_default()).ok_or_else(
                || error_msg)?
            + chrono::TimeDelta::try_seconds(self.seconds.unwrap_or_default()).ok_or_else(
                || error_msg)?
        )
    }
}

//These tests ensure that configurations can be converted to and from .json files.
#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_JSON: &str =
r#"{
    "trash": {
        "is_enabled": true,
        "time_limit": {
            "weeks": 2,
        }
    },
    "folders": []
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

