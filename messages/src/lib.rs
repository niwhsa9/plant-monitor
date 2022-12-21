pub mod msg {

    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }
}