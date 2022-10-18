use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::*;

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
    
    spawn_local( run() );
    yew::start_app::<App>();
    log::info!("Some info");
}

#[wasm_bindgen]
pub async fn run() {
    log::info!("here");
    //let client = reqwest::Client::new();
    //client.get("/api").send().await.unwrap();
    Request::get("/api")
        .send()
        .await
        .unwrap();
    //match res {
    //    Ok(_) => log::info!("good"),
    //    Err(_) => log::error!("bad")
    //};
    log::info!("donea");


}