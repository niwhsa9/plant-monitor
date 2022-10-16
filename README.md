# Plant Monitor
This is a web application to monitor the health of your plants

## Current Features

## Technical Overview
* This application is written purely in Rust from end-to-end
* Rust WASM via Yew is used for the front-end 
* The server side application stores data with SQLite
* Messages are serialized into JSON with Serde to pass around through HTTP 
* Trunk is used to deploy WASM

## Planned Features
* View the status of the last watering date of your plant 
* Upload images to track the growth of your plants over time
* Optionally use an ESP8266 with a capactive moisture sensor to provide highly accurate soil moisture calibrations

## Dev setup 
1. Git clone this repository
2. Install Rust
3. Get trunk `cargo install trunk`
4. Add WASM target `rustup target add wasm32-unknown-unknown`
5. Run the server: `cargo run --bin server`