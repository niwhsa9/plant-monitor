# Plant Monitor
This is a web application to monitor the health of your plants

## Current Features
* View the status of the last watering date of your plant 

## Technical Overview
* JavaScript sucks
* This application is written purely in Rust from end-to-end
* Rust WASM via Yew is used for the front-end 
* The server side application stores data with SQLite
* Warp (with Tokio) is used to map api requests 
* Messages are serialized into JSON with Serde to pass around through HTTP 
* Trunk is used to deploy WASM

## Planned Features
* Plant health status indicator based on callibrated watering schedule
* Growth tracking of your plants over time
* Integration with moisture sensors for calibrations and more accurate status

## Installation
TODO

## Dev setup 
1. Git clone this repository
2. Install Rust
3. Get trunk `cargo install trunk`
4. Add WASM target `rustup target add wasm32-unknown-unknown`
5. Run the server: `cargo run --bin plant-monitor-server`
6. Run the client: `cd client && trunk serve --open`


## Credits:
* DateTime serialization: https://serde.rs/custom-date-format.html
* Stylesheets for buttons and top bar: https://www.w3schools.com/css/default.asp 