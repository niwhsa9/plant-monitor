use warp::Filter;

#[tokio::main]
async fn main() {
    println!("Hello from server!");
    let routes = warp::any().map(|| {
        println!("get req");
         "Hello World"
    });
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
