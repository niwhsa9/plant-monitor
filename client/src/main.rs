use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::*;

/*
 * Plant Display Widget
 */
#[derive(Clone, Properties, PartialEq)]
pub struct PlantWidgetProps {
    pub name: String,
}

#[function_component]
fn PlantWidget(props : &PlantWidgetProps) -> Html {
    log::info!("Widget rerender");
    html! {
        <div class="plant-widget">
            <h1>{&props.name}</h1>
        </div>
    }
}

/*
 * Top Level Application Dashboard
 */
pub struct Dashboard {
    plants : Vec<PlantWidgetProps>
}

impl Component for Dashboard {
    type Message = ();
    type Properties = ();

    fn create(ctx : &Context<Self>) -> Self {
        // Load application information from server 

        // Request an image from the API for the particular plant
        // Get the resource URL for the image tag
        let plant_widgets = vec![ 
            PlantWidgetProps{name : String::from("Claude")}, 
            PlantWidgetProps{ name : String::from("Jacobi")} 
        ];
        Self{plants : plant_widgets}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <> 
            {self.plants.iter().map(|plant| { html! {<PlantWidget ..plant.clone()/>} }).collect::<Html>()}
            </>
        }
    }

}


fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    spawn_local( run() );
    yew::Renderer::<Dashboard>::new().render();

}

pub async fn run() {
    Request::get("/api")
        .send()
        .await
        .unwrap();
}