use std::{sync::Arc, sync::Mutex};

use warp::{Filter};
use messages::msg::{PlantData};
use chrono::{Utc, TimeZone};

const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

#[tokio::main]
async fn main() {
    println!("Server starting");
    let mut data = Vec::<PlantData>::new();

    // Issue initial database reads and cache
    let connection = Arc::new(Mutex::new(sqlite::open("plants.db").unwrap()));
    let query = "
        CREATE TABLE IF NOT EXISTS plants (name TEXT, img TEXT, water DATETIME, PRIMARY KEY (name));
        INSERT OR IGNORE INTO plants VALUES ('Claude', '/api/img/plant.jpg', '2022-09-24 09:05:00' );
        INSERT OR IGNORE INTO plants VALUES ('Jacobi', '/api/img/plant.jpg', '2022-09-25 09:05:00' );
    ";
    connection.lock().unwrap().execute(query).unwrap();
    
    let query = "SELECT * from plants";
    for row in connection.lock().unwrap()
        .prepare(query)
        .unwrap()
        .into_iter()
        .map(|row| row.unwrap())
    {
        let e = PlantData {
            name : String::from(row.read::<&str, _>("name")),
            img_path : String::from(row.read::<&str, _>("img")),
            last_water_time : Utc.datetime_from_str(
                row.read::<&str, _>("water"), FORMAT).unwrap(),
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
       map(move|name : String| {
        let new_time = Utc::now();
        let new_time_str = new_time.format(FORMAT);
        // Update cache
        //data_ptr[0].name = String::from("LOl");

        // Write through to disk as well
        let query = format!("
            UPDATE plants SET water='{}' where name='{}'; ",
            new_time_str, name
        );
        connection.lock().unwrap().execute(query).unwrap();
        warp::reply()
        });

    // Serve API endpoints    
    let routes = 
        (warp::get().and(
            data_route
            .or(image_route)
        )).or(warp::post().and(
            reset_time_route
        ));
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
