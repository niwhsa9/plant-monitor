use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::*;
use messages::msg::{PlantData};

/*
 * Plant Display Widget
 */
#[function_component]
fn PlantWidget(props : &messages::msg::PlantData) -> Html {
    let s = (&props).img_path.clone();
    html! {
        <div class="plant-widget">
            <h1>{&props.name}</h1>
            <img src={s}/> 
            <p>{String::from("Last watered: ")}</p>
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
                self.plants = Some(data);
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
                {p.iter().map(|plant| { html! {<PlantWidget ..plant.clone()/>} }).collect::<Html>()}
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