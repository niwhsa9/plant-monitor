
pub mod msg {

    // source: https://serde.rs/custom-date-format.html
    mod datetime_format {
        use chrono::{DateTime, Utc, TimeZone};
        use serde::{self, Deserialize, Serializer, Deserializer};

        const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

        pub fn serialize<S>(
            date: &DateTime<Utc>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let s = format!("{}", date.format(FORMAT));
            serializer.serialize_str(&s)
        }

        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
        }
    }

    use serde::{Serialize, Deserialize};
    use yew::prelude::*;
    use chrono::{DateTime, Utc};

    #[derive(Serialize, Deserialize, Debug, Properties, PartialEq, Clone)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub struct PlantData {
        pub name : String,
        pub img_path : String,
        #[serde(with = "datetime_format")]
        pub last_water_time : DateTime<Utc>
    }
}
