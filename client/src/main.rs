use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

//#[wasm_bindgen]
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Some info");

    println!("trying to spawn local");
    spawn_local( run() );
    yew::start_app::<App>();
    log::info!("Some info");
}

#[wasm_bindgen]
pub async fn run() {
    log::info!("here");
    //let client = reqwest::Client::new();
    let res = reqwest::get("/api").await;//client.get("localhost:3030").send().await;
    match res {
        Ok(_) => log::info!("good"),
        Err(_) => log::error!("bad")
    };
    log::info!("done");


}