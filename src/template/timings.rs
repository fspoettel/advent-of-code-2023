use std::{collections::HashMap, fs, io::Error, str::FromStr};
use tinyjson::JsonValue;

use super::Day;

/// Represents benchmark times for a single day.
#[derive(Clone, Debug)]
pub struct Timing {
    pub day: Day,
    pub part_1: Option<String>,
    pub part_2: Option<String>,
    pub total_nanos: f64,
}

/// Represents benchmark times for a set of days.
/// Can be serialized from / to JSON.
#[derive(Clone, Debug, Default)]
pub struct Timings {
    pub data: Vec<Timing>,
}

impl Timings {
    /// Dehydrate timings to a JSON file.
    pub fn store_file(&self) -> Result<(), Error> {
        let json = JsonValue::from(self.clone());
        let mut bytes = vec![];
        json.format_to(&mut bytes)?;
        fs::write("./data/timings.json", bytes)
    }

    /// Rehydrate timings from a JSON file. If not present, returns empty timings.
    pub fn read_from_file() -> Self {
        let s = fs::read_to_string("./data/timings.json")
            .map_err(|x| x.to_string())
            .and_then(Timings::try_from);

        match s {
            Ok(timings) => timings,
            Err(e) => {
                eprintln!("{}", e);
                Timings::default()
            }
        }
    }

    /// Merge two sets of timings, overwriting `self` with `other` if present.
    pub fn merge(&self, new: &Self) -> Self {
        let mut data: Vec<Timing> = vec![];

        for timing in &new.data {
            data.push(timing.clone());
        }

        for timing in &self.data {
            if !data.iter().any(|t| t.day == timing.day) {
                data.push(timing.clone());
            }
        }

        data.sort_unstable_by(|a, b| a.day.cmp(&b.day));
        Timings { data }
    }

    /// Sum up total duration of timings as millis.
    pub fn total_millis(&self) -> f64 {
        self.data.iter().map(|x| x.total_nanos).sum::<f64>() / 1_000_000_f64
    }
}

/* -------------------------------------------------------------------------- */

impl From<Timings> for JsonValue {
    fn from(value: Timings) -> Self {
        let mut map: HashMap<String, JsonValue> = HashMap::new();

        map.insert(
            "data".into(),
            JsonValue::Array(value.data.iter().map(JsonValue::from).collect()),
        );

        JsonValue::Object(map)
    }
}

impl TryFrom<String> for Timings {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let json = JsonValue::from_str(&value).or(Err("not valid JSON file."))?;

        let json_data = json
            .get::<HashMap<String, JsonValue>>()
            .ok_or("expected JSON document to be an object.")?
            .get("data")
            .ok_or("expected JSON document to have key `data`.")?
            .get::<Vec<JsonValue>>()
            .ok_or("expected `json.data` to be an array.")?;

        Ok(Timings {
            data: json_data
                .iter()
                .filter_map(|value| Timing::try_from(value).ok())
                .collect(),
        })
    }
}

/* -------------------------------------------------------------------------- */

impl From<&Timing> for JsonValue {
    fn from(value: &Timing) -> Self {
        let mut map: HashMap<String, JsonValue> = HashMap::new();

        map.insert("day".into(), JsonValue::String(value.day.to_string()));
        map.insert("total_nanos".into(), JsonValue::Number(value.total_nanos));

        let part_1 = value.part_1.clone().map(JsonValue::String);
        let part_2 = value.part_2.clone().map(JsonValue::String);

        map.insert(
            "part_1".into(),
            match part_1 {
                Some(x) => x,
                None => JsonValue::Null,
            },
        );

        map.insert(
            "part_2".into(),
            match part_2 {
                Some(x) => x,
                None => JsonValue::Null,
            },
        );

        JsonValue::Object(map)
    }
}

impl TryFrom<&JsonValue> for Timing {
    type Error = String;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        let json = value
            .get::<HashMap<String, JsonValue>>()
            .ok_or("Expected timing to be a JSON object.")?;

        let day = json
            .get("day")
            .and_then(|v| v.get::<String>())
            .and_then(|day| Day::from_str(day).ok())
            .ok_or("Expected timing.day to be a Day struct.")?;

        let part_1 = json
            .get("part_1")
            .and_then(|v| {
                if v.is_null() {
                    None
                } else {
                    Some(v.get::<String>())
                }
            })
            .ok_or("Expected timing.part_1 to be null or string.")?;

        let part_2 = json
            .get("part_2")
            .and_then(|v| {
                if v.is_null() {
                    None
                } else {
                    Some(v.get::<String>())
                }
            })
            .ok_or("Expected timing.part_2 to be null or string.")?;

        let total_nanos = json
            .get("total_nanos")
            .and_then(|v| v.get::<f64>().copied())
            .ok_or("Expected timing.total_nanos to be a number.")?;

        Ok(Timing {
            day,
            part_1: part_1.cloned(),
            part_2: part_2.cloned(),
            total_nanos,
        })
    }
}
