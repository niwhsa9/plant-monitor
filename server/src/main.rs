use warp::{Filter};
use messages::msg::{Point, PlantData};
use chrono::{DateTime, Utc, TimeZone, NaiveDate};

mod handlers;

#[tokio::main]
async fn main() {
    println!("Server starting");
    // Issue database transaction and cache

    let data_route = 
        warp::path("plant_data").map( || { 
            // [Temp] construct vector of Plant Data
            let naivedatetime_utc = NaiveDate::from_ymd_opt(2000, 1, 12).unwrap().and_hms_opt(2, 0, 0).unwrap();
            let datetime_utc = DateTime::<Utc>::from_utc(naivedatetime_utc, Utc); 
            let cur_time: DateTime<Utc> = Utc::now();
            let p = vec![
                PlantData{ name : String::from("Wernher"), img_path : String::from("/api/img/plant.jpg"), last_water_time : cur_time},
                PlantData{ name : String::from("Wernher1"), img_path : String::from("/api/img/plant.jpg"), last_water_time : datetime_utc},
                PlantData{ name : String::from("Wernher2"), img_path : String::from("/api/img/plant.jpg"), last_water_time : cur_time},
                PlantData{ name : String::from("Wernher3"), img_path : String::from("/api/img/plant.jpg"), last_water_time : cur_time},
            ];
            // Return data
            warp::reply::json(&p)
            }
        );
    let image_route = warp::path("img") .and(warp::fs::dir("temp"));

    let test_route = 
        warp::path("test").map( || {
            println!("Test triggered");
            //"Test Reply" 
            let p = Point{ x: 1, y: 2};
            warp::reply::json(&p)
        });

    let routes = 
        warp::get().and(
            data_route
            .or(test_route)
            .or(image_route)
        );

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
