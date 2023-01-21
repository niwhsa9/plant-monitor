use std::{sync::Arc, sync::Mutex, borrow::BorrowMut, path::Path, fs};
use warp::{Filter, multipart, hyper::Uri};
use messages::msg::{PlantData};
use chrono::{Utc, TimeZone};
use bytes::BufMut;
use futures_util::{TryFutureExt, TryStreamExt};

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
    let data_ptr = Arc::new(Mutex::new(data));
    let data_ptr_clone = data_ptr.clone();
    // Return data about all available plants
    let data_route = 
        warp::path("plant_data").map( move || { 
            warp::reply::json(&*(data_ptr.lock().unwrap()))
            }
        );
    let image_route = warp::path("img") .and(warp::fs::dir("temp"));

    // Handle a request to reset watering time of a plant
    let reset_time_route = 
       warp::path("reset_time").
       and(warp::path::param::<String>()).
       map(move|name : String| {
        let new_time = Utc::now();
        let new_time_str = new_time.format(FORMAT);

        // Update cache
        let mut data = data_ptr_clone.lock().unwrap();
        let mut i = data.iter_mut();
        let res = i.find(|x| x.name == name ).unwrap();
        res.last_water_time = new_time;

        // Write through to disk as well
        let query = format!("
            UPDATE plants SET water='{}' where name='{}'; ",
            new_time_str, name
        );
        connection.lock().unwrap().execute(query).unwrap();
        warp::reply()
        });

    // Add new plant form endpoint
    
    let new_plant = multipart::form().and_then(|form: multipart::FormData| {
            async {
            // Collect the fields into (name, value): (String, Vec<u8>), taken from warp examples
            let part: Result<Vec<(String, Vec<u8>)>, warp::Rejection> = form
                .and_then(|part| {
                    let name = part.name().to_string();
                    let value = part.stream().try_fold(Vec::new(), |mut vec, data| {
                        vec.put(data);
                        async move { Ok(vec) }
                    });
                    value.map_ok(move |vec| (name, vec))
                })
                .try_collect()
                .await
                .map_err(|e| {
                    panic!("multipart error: {:?}", e);
                });
            part
            }
        }).map(|v : Vec<(String, Vec<u8>)>| { 
            for p in v.iter() {
                let (name, data) = p; 
                match name.as_str() {
                    "plant_name" => {
                        println!("recieved {} {}", name, String::from_utf8(data.to_vec()).unwrap());
                    },
                    "fname" => {
                        println!("img size {}", data.len());
                        //image::save_buffer("image.jpg", data, 500, 500, image::ColorType::Rgb8).unwrap()
                        let path = Path::new("image.jpg");
                        fs::write(path, data).unwrap();
                    }
                    _ => panic!()
                }
                //println!("recieved {} {}", name, String::from_utf8(data.to_vec()).unwrap());
            }
            warp::reply()
        });

    // Serve API endpoints    
    let routes = 
        //warp::body::content_length_limit(100000).and
        (warp::get().and(
            data_route
            .or(image_route)
        )).or(warp::post().and(
            reset_time_route
        ).or(new_plant) ); //.or(new_plant));
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
