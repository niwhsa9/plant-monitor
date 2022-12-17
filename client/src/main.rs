use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::*;

#[function_component(App)]
fn app() -> Html {
    // Initial request to determine the number of widgets
    html! {
        <> 
        <PlantWidget/>
        <PlantWidget/>
        <PlantWidget/>
        </>
    }
}

pub struct PlantWidget {
    name : String  
}

impl Component for PlantWidget {
    type Message = ();
    type Properties = ();

    fn create(ctx : &Context<Self>) -> Self {
        Self{name : String::from("lol")}
    }

    fn update(&mut self, ctx : &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Get the data from the server for the widget
        // Request an image from the API for the particular plant
        // Get the resource URL for the image tag
    
        html! {
            <div class="plant-widget">
                <h1>{&self.name}</h1>
            </div>
        }
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
    Request::get("/api")
        .send()
        .await
        .unwrap();
    log::info!("donea");
}