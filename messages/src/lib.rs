pub mod msg {

    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Point {
        x: i32,
        y: i32,
    }
}