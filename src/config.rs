use chrono::TimeDelta;

#[derive(PartialEq, Eq, Debug)]
struct Config {
    time_limit: chrono::TimeDelta
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
            result.time_limit = TimeDelta::try_seconds(seconds).ok_or_else(|| "time too large!")?;
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
        json.insert(String::from("time_limit"), serde_json::Value::Object(time_limit));

        let result = serde_json::Value::Object(json);
        let result = serde_json::to_string_pretty(&result)?;
        Ok(result)
    }
}

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

