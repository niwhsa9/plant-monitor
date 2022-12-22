pub mod msg {
    use serde::{Serialize, Deserialize};
    use yew::prelude::*;
    //use chrono::{DateTime};

    #[derive(Serialize, Deserialize, Debug, Properties, PartialEq, Clone)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    #[derive(Serialize, Deserialize, Debug, Properties, PartialEq, Clone)]
    pub struct PlantData {
        pub name : String,
        pub img_path : String
    }
}