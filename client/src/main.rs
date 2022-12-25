use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::*;
use messages::msg::{PlantData};
use std::rc::{Rc};
use chrono::{DateTime, Local, Utc};
use std::cell::RefCell;

/*
 * Plant Display Widget
 */

#[derive(Properties, PartialEq, Clone)]
pub struct PlantWidgetProps {
    // TODO: this one really could be a Weak
    // Mutable ref counted pointer is necessary here becasue 
    // Yew does not support lifetime annotation in function
    // components so single-ownership with non-owning 
    // references is not valid. 
    // TODO: Maybe refactor to use Yew's state hooks instead
    // Each component can then own its own data
    plant_data : PlantData
}


#[function_component]
fn PlantWidget(props : &PlantWidgetProps) -> Html {
    let plant_data = use_state(|| props.plant_data.clone());

    let cur_time: DateTime<Local> = Local::now();
    let date_local : DateTime<Local> = DateTime::from(plant_data.last_water_time);
    let diff = cur_time - date_local;
    let date_str = 
        if diff.num_days() > 0 { 
            format!("{} days", diff.num_days()) 
        } else { 
            format!("{} hours", diff.num_hours()) 
        };

    let reset_cb = Callback::from(move |_ : MouseEvent| {
            log::info!("here");
                plant_data.last_water_time = Utc::now();
                ()
            } );

    html! {
        <div class="plant-widget">
            <h1>{plant_data.name}</h1>
            <img src={plant_data.img_path}/> 

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
    // TODO: refactor out option since we can check empty vec
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