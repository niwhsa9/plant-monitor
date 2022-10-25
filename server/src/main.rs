use warp::{Filter};
mod handlers;

mod filters {
    use super::handlers;
    use warp::{Filter};
    pub fn get_plants() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("plants").map(handlers::get_plants())
    }
}

#[tokio::main]
async fn main() {
    println!("Server starting");


    let routes = warp::any().map(|| {
        println!("get req");
        //#Response::builder().header("test", "val").body("lol")
        "hello world"
    });
    handlers::get_plants();    
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
