use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::*;
use messages::msg::{Point};

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
    loading : Box<bool>,
    plants : Vec<PlantWidgetProps>,
}

pub enum DashboardMsg {
    DataReady()
}
impl Dashboard {
    // Retrieves plant data from the server for the application 
    // and issues callback on completion
    fn get_plant_data(completed_cb : Callback<(), ()>) {
        spawn_local( async move { 
            // HTTP GET data from server
            let r = Request::get("/api/test")
                .send()
                .await
                .unwrap();

            // Signal completion
            completed_cb.emit(());      
        });
    }
}

impl Component for Dashboard {
    type Message = DashboardMsg;
    type Properties = ();

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::DataReady() => {

            }
        }
        false
    }

    fn create(ctx : &Context<Self>) -> Self {
        // Load application information from server 
        let loading = Box::new(true);
        let mut l2 = loading.clone();
        let q = async move { 
            *l2 = false; 
            
        };
        spawn_local( q );
        // Request an image from the API for the particular plant
        // Get the resource URL for the image tag
        let plant_widgets = vec![ 
            PlantWidgetProps{name : String::from("Claude")}, 
            PlantWidgetProps{ name : String::from("Jacobi")} 
        ];
        Self{loading : Box::new(true), plants : plant_widgets}
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
    // Get the data we need from the server, it will callback 
    spawn_local( run() );
    yew::Renderer::<Dashboard>::new().render();

}

pub async fn run() {
    let r = Request::get("/api/test")
        .send()
        .await
        .unwrap();
    //log::info!("{}", r.text().await.unwrap());
    let p = r.json::<Point>().await.unwrap();
    log::info!("px {} py {}", p.x, p.y);
}