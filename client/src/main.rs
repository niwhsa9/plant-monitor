use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::*;

pub struct Dashboard {
    num_widgets : u32,
}

// TODO: change this to regular Component and do server transaction in create() to avoid repeat
#[function_component(App)]
fn app() -> Html {
    // Initial request to determine the number of widgets
    let v = vec![1, 2, 3, 4, 5];
    html! {
        <> 
        {v.into_iter().map(|_id| {html!{ <PlantWidget/>} } ).collect::<Html>()}
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
        log::info!("Widget create");
        Self{name : String::from("lol")}
    }

    fn update(&mut self, ctx : &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Get the data from the server for the widget
        // Request an image from the API for the particular plant
        // Get the resource URL for the image tag
    
        log::info!("Widget rerender");
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
    spawn_local( run() );
    yew::start_app::<App>();
}

#[wasm_bindgen]
pub async fn run() {
    Request::get("/api")
        .send()
        .await
        .unwrap();
}