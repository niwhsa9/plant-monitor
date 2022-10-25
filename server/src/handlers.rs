use serde::{Serialize, Deserialize};
use warp::http::StatusCode;
use std::convert::Infallible;

#[derive(Serialize, Deserialize, Debug)]
struct PlantData {
    name : String,
    image_path : String,
    //last_water : DateTime<Local>
}

pub async fn get_plants() -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}