use std::{fs, io::{Read, Write}, path::PathBuf};
use chrono::TimeDelta;

#[derive(PartialEq, Eq, Debug)]
pub struct Config {
    pub time_limit: chrono::TimeDelta
}

impl Config {
    const TIME_UNITS: [&str; 5]  = ["weeks", "days", "hours", "minutes", "seconds"];
    const TIME_UNITS_AS_SECONDS: [i64; 5] = [60 * 60 * 24 * 7, 60 * 60 * 24, 60 * 60, 60, 1];

    pub fn default() -> Config {
        Config {
            time_limit: TimeDelta::weeks(2)
        }
    }

    pub fn from_json(json: &str) -> Result<Config, Box<dyn std::error::Error>>{
        let json: serde_json::Value = serde_json::from_str(json)?;
        let mut result = Config::default();
        let time_limit = &json["time_limit"];

        //In config.json, time_limit is stored as an object containing fields for different units.
        //Therefore, we have to read each unit to get the complete TimeDelta.
        if !time_limit.is_null() {
            result.time_limit = TimeDelta::zero();
            let mut seconds = 0;
            for i in 0..Config::TIME_UNITS.len() {
                let unit = Config::TIME_UNITS[i];
                let unit_value = &time_limit[unit];
                if unit_value.is_null() {
                    continue;
                }

                let error_msg = format!("'{unit}' is not an i64");
                let unit_value = unit_value.as_number().ok_or_else(|| error_msg.clone())?;
                let unit_value = unit_value.as_i64().ok_or_else(|| error_msg.clone())?;
                seconds += Config::TIME_UNITS_AS_SECONDS[i] * unit_value;
            }
            result.time_limit = TimeDelta::try_seconds(seconds).ok_or_else(|| "Time too large!")?;
        }

        Ok(result)
    }

    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut json = serde_json::Map::new();
        let mut time_limit = serde_json::Map::new();

        time_limit.insert(String::from("weeks"), serde_json::Value::Number(
                serde_json::Number::from(self.time_limit.num_weeks())));
        time_limit.insert(String::from("days"), serde_json::Value::Number(
                serde_json::Number::from(self.time_limit.num_days() % 7)));
        time_limit.insert(String::from("hours"), serde_json::Value::Number(
                serde_json::Number::from(self.time_limit.num_hours() % 24)));
        time_limit.insert(String::from("minutes"), serde_json::Value::Number(
                serde_json::Number::from(self.time_limit.num_minutes() % 60)));
        time_limit.insert(String::from("seconds"), serde_json::Value::Number(
                serde_json::Number::from(self.time_limit.num_seconds() % 60)));

        json.insert(String::from("time_limit"),
            serde_json::Value::Object(time_limit));

        let result = serde_json::Value::Object(json);
        let result = serde_json::to_string_pretty(&result)?;
        Ok(result)
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

    const DEFAULT_JSON: &str = r#"{
    "time_limit": {
        "weeks": 2,
        "days": 0,
        "hours": 0,
        "minutes": 0,
        "seconds": 0
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

