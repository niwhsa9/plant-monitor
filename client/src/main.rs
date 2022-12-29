use std::rc::Rc;

use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::*;
use messages::msg::{PlantData};
use chrono::{DateTime, Local, Utc};

/*
 * Plant Display Widget
 */

#[derive(Properties, PartialEq, Clone)]
pub struct PlantWidgetProps {
    plant_data : PlantData
}


#[function_component]
fn PlantWidget(props : &PlantWidgetProps) -> Html {
    let last_water_time = use_state(|| props.plant_data.last_water_time);

    let cur_time: DateTime<Local> = Local::now();
    let date_local : DateTime<Local> = DateTime::from(*last_water_time);
    let diff = cur_time - date_local;
    let date_str = 
        if diff.num_days() > 0 { 
            format!("{} days", diff.num_days()) 
        } else { 
            format!("{} hours", diff.num_hours()) 
            //format!("{} hours", diff.num_seconds()) 
        };

    // Button callback
    let name = props.plant_data.name.clone();
    let reset_cb = Callback::from(move |_ : MouseEvent| {
            last_water_time.set(Utc::now());
            // indirection is necessary here due to lack of syntactic sugar for
            // capture by clone
            let name = name.clone();
            let endpt = format!("/api/reset_time/{}", name);
            spawn_local( async move { 
                            let r = Request::post(&endpt)
                                .body(name.clone())
                                .send()
                                .await;
                        });
                        
            ()
            } );

    html! {
        <div class="plant-widget">
            <h1>{props.plant_data.name.clone()}</h1>
            <img src={props.plant_data.img_path.clone()}/> 

            <button onclick={reset_cb}>
                { "Reset" }
            </button>
            <p>
                { (String::from("Last watered: ") + &date_str) }
            </p>
        </div>
    }
}

/*
 * Top Level Application Dashboard
 */
pub struct Dashboard {
    // TODO: Refactor out - API design is poor, if API request is per plant
    // then each PlantWidget may request its own data in create() and maintain
    // its own internal state
    plants : Vec<PlantData>
}
pub enum DashboardMsg {
    DataReady(Vec<PlantData>)
}
impl Dashboard {
    // Retrieves plant data from the server for the application 
    // and issues callback on completion
    pub fn get_plant_data(&self, completed_cb : Callback<Vec<PlantData>>) {
        spawn_local( async move { 
            // HTTP get data array from server
            let r = Request::get("/api/plant_data")
                .send()
                .await
                .unwrap().json::<Vec<PlantData>>().await.unwrap();

            // Signal completion
            completed_cb.emit(r);      
        });
    }
}
impl Component for Dashboard {
    type Message = DashboardMsg;
    type Properties = ();

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::DataReady(data) => {
                self.plants = data;
                return true;
            }
        }
    }

    fn create(ctx : &Context<Self>) -> Self {
        // Create the dashboard, register callback to populate data, and dispatch GET
        let dash = Self{plants : vec![]};
        let data_cb = ctx.link().callback(Self::Message::DataReady);
        dash.get_plant_data(data_cb);
        return dash
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.plants.len() {
            // Display loading screen while waiting for GET
            0 => html! {
                <>
                <p>{"Loading... "}</p>
                </>
            },
            // Display widgets when they are available
            _ => html! {
                <> 
                <div class="topbar">
                    <a class="active" href="#home">{String::from("Home")}</a>
                    <a href="#home">{String::from("Data")}</a>
                </div>
                <div class="widgets-grid">
                    {self.plants.iter().map(|plant| { html! {<PlantWidget plant_data={plant.clone()}/>} }).collect::<Html>()}
                </div>
                </>
            }
        } 
    }

}


fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Dashboard>::new().render();
}