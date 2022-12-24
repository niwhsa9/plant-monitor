use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::*;
use messages::msg::{PlantData};
use std::rc::{Rc};
/*
 * Plant Display Widget
 */

#[derive(Properties, PartialEq, Clone)]
pub struct PlantWidgetProps {
    // TODO: this one really could be a Weak
    plant_data : Rc<PlantData> 
}

#[function_component]
fn PlantWidget(props : &PlantWidgetProps) -> Html {
    let s = props.plant_data.img_path.clone();
    html! {
        <div class="plant-widget">
            <h1>{&props.plant_data.name}</h1>
            <img src={s}/> 
            <p>{String::from("Last watered: ")}</p>
        </div>
    }
}

/*
 * Top Level Application Dashboard
 */
pub struct Dashboard {
    plants : Option<Vec<Rc<PlantData>>>
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
                // Need to convert Vec<PlantData> into Vec<Rc<PlantData>>
                let v = data.into_iter().map(|x| { Rc::new(x) } ).collect();
                self.plants = Some(v);
                return true;
            }
        }
    }

    fn create(ctx : &Context<Self>) -> Self {
        // Create the dashboard, register callback to populate data, and dispatch GET
        let dash = Self{plants : None};
        let data_cb = ctx.link().callback(Self::Message::DataReady);
        dash.get_plant_data(data_cb);
        return dash
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.plants {
            Some(p) => html! {
                <> 
                <div class="topbar">
                    <a class="active" href="#home">{String::from("Home")}</a>
                    <a href="#home">{String::from("Data")}</a>
                </div>
                <div class="widgets-grid">
                    {p.iter().map(|plant| { html! {<PlantWidget plant_data={plant}/>} }).collect::<Html>()}
                </div>
                </>
            },
            // Display loading screen while waiting for GET
            None => html! {
                <>
                <p>{"Loading... "}</p>
                </>
            }
        } 
    }

}


fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Dashboard>::new().render();
}