use std::{rc::Rc, sync::Arc};

use warp::{Filter};
use messages::msg::{Point, PlantData};
use chrono::{DateTime, Utc, TimeZone, NaiveDate};

#[tokio::main]
async fn main() {
    println!("Server starting");
    let mut data = Vec::<PlantData>::new();

    // Issue database transactions and cache
    let connection = sqlite::open("plants.db").unwrap();
    let query = "
        CREATE TABLE IF NOT EXISTS plants (name TEXT, img TEXT, water DATETIME, PRIMARY KEY (name));
        INSERT OR IGNORE INTO plants VALUES ('Claude', '/api/img/plant.jpg', '2022-09-24 09:05:00' );
        INSERT OR IGNORE INTO plants VALUES ('Jacobi', '/api/img/plant.jpg', '2022-09-25 09:05:00' );
    ";
    connection.execute(query).unwrap();
    
    let query = "SELECT * from plants";
    for row in connection
        .prepare(query)
        .unwrap()
        .into_iter()
        .map(|row| row.unwrap())
    {
        let e = PlantData {
            name : String::from(row.read::<&str, _>("name")),
            img_path : String::from(row.read::<&str, _>("img")),
            last_water_time : Utc.datetime_from_str(
                row.read::<&str, _>("water"), "%Y-%m-%d %H:%M:%S").unwrap(),
        };
        data.push(e);
    }
    let data_ptr = Arc::new(data);

    let data_route = 
        warp::path("plant_data").map( move || { 
            warp::reply::json(&*data_ptr)
            }
        );
    let image_route = warp::path("img") .and(warp::fs::dir("temp"));

    let reset_time_route = 
       warp::path("reset_time").
       and(warp::path::param::<String>()).
       map(|param : String| { println!("needs reset {}", param); warp::reply()});
        
    let routes = 
        (warp::get().and(
            data_route
            .or(image_route)
        )).or(warp::post().and(
            reset_time_route
        ));

        /* 
    let routes = 
        (warp::get().and(
            data_route
            .or(image_route)
        )).or(warp::post().map(warp::reply));
        */

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
