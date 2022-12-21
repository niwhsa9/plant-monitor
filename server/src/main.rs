use warp::{Filter};
use messages::msg::{Point};

mod handlers;

#[tokio::main]
async fn main() {
    println!("Server starting");

    let data_route = 
        warp::path("plant_data").map( || "Response" );

    let test_route = 
        warp::path("test").map( || "Test Reply" );

    let routes = 
        warp::get().and(
            data_route
            .or(test_route)
        );

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
