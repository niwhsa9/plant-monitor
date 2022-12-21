use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::*;
use messages::msg::{Point, PlantData};

/*
 * Plant Display Widget
 */
#[function_component]
fn PlantWidget(props : &messages::msg::PlantData) -> Html {
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
    plants : Option<Vec<PlantData>>
}

pub enum DashboardMsg {
    DataReady(Vec<PlantData>)
}
impl Dashboard {
    // Retrieves plant data from the server for the application 
    // and issues callback on completion
    pub fn get_plant_data(&self, completed_cb : Callback<Vec<PlantData>>) {
        spawn_local( async move { 
            // HTTP GET data from server
            let r = Request::get("/api/plant_data")
                .send()
                .await
                .unwrap().json::<Vec<PlantData>>().await.unwrap();

            println!("{}", r[0].name);
            // Signal completion
            //completed_cb.emit(());      
        });
    }
}

impl Component for Dashboard {
    type Message = DashboardMsg;
    type Properties = ();

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::DataReady(data) => {

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
            PlantData{name : String::from("Claude"), img_path : String::from("")}, 
            PlantData{name : String::from("Jacobi"), img_path : String::from("")} 
        ];
        let dash = Self{plants : Some(plant_widgets)};
        let data_cb = ctx.link().callback(Self::Message::DataReady);
        //dash.get_plant_data(data_cb);
        dash
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.plants {
            Some(p) => html! {
                <> 
                {p.iter().map(|plant| { html! {<PlantWidget ..plant.clone()/>} }).collect::<Html>()}
                </>
            },
            None => html! {}
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