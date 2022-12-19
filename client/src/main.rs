use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::*;

/*
 * Plant Display Widget
 */
#[derive(Properties, PartialEq)]
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
    num_plants : u32,
    plants : Vec<PlantWidgetProps>
}

impl Component for Dashboard {
    type Message = ();
    type Properties = ();

    fn create(ctx : &Context<Self>) -> Self {
        // Load application information from server 
        let plant_widgets = vec![ 
            PlantWidgetProps{name : String::from("Claude")}, 
            PlantWidgetProps{ name : String::from("Jacobi")} 
        ];
        Self{num_plants : 0, plants : plant_widgets}
    }

    fn update(&mut self, ctx : &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Get the data from the server for the widget
        // Request an image from the API for the particular plant
        // Get the resource URL for the image tag
    
        log::info!("Widget rerender");
        let v = vec![1, 2, 3, 4, 5];
        html! {
            <> 
            {v.into_iter().map(|_id| {html!{ <PlantWidget name="Plant 1"/>} } ).collect::<Html>()}
            </>
        }
    }

}


//#[wasm_bindgen]
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    spawn_local( run() );
    yew::Renderer::<Dashboard>::new().render();

}

#[wasm_bindgen]
pub async fn run() {
    Request::get("/api")
        .send()
        .await
        .unwrap();
}