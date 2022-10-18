use warp::{Filter, http::Response};

#[tokio::main]
async fn main() {
    println!("Hello from server!");
    let routes = warp::any().map(|| {
        println!("get req");
        //#Response::builder().header("test", "val").body("lol")
        "hello world"
    });
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
