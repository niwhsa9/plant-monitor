use warp::{Filter};
use messages::msg::{Point};

mod handlers;

#[tokio::main]
async fn main() {
    println!("Server starting");

    let data_route = 
        warp::path("plant_data").map( || "Response" );

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
        );

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
