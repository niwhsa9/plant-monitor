use warp::{Filter};
use messages::msg::{Point, PlantData};

mod handlers;

#[tokio::main]
async fn main() {
    println!("Server starting");
    // Issue database transaction and cache

    let data_route = 
        warp::path("plant_data").map( || { 
            // [Temp] construct vector of Plant Data
            let p = vec![
                PlantData{ name : String::from("Wernher"), img_path : String::from("/api/img/plant.jpg")},
                PlantData{ name : String::from("Claude"), img_path : String::from("/api/img/plant2.jpg")},
                PlantData{ name : String::from("Wernher2"), img_path : String::from("/api/img/plant.jpg")},
                PlantData{ name : String::from("Wernher3"), img_path : String::from("/api/img/plant.jpg")},
                PlantData{ name : String::from("Wernher3"), img_path : String::from("/api/img/plant.jpg")},
                PlantData{ name : String::from("Wernher3"), img_path : String::from("/api/img/plant.jpg")},
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
