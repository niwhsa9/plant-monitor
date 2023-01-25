# Plant Monitor
This is a web application to monitor your plants

![Demo](https://github.com/niwhsa9/plant-monitor/blob/master/demo.png?raw=true)

I mainly wrote this because I wanted to try WASM in Rust.

## Current Features
* View and track the last watering date of all of your plants

## Technical Overview
* This web application is written in Rust from end-to-end because Javascript and more generally, dynamic typing, suck
* Rust WASM and Yew is used for the front-end, and deployed with Trunk
* The server stores plant data in a SQLite db
* Messages are serialized into JSON with Serde to pass around through HTTP 

## Potential Future Features
* Plant health indicator based on pre-callibrated watering schedule
* Moisture sensor integration 

## Release Installation
TODO

## Dev Installation 
1. Git clone this repository
2. Install Rust
3. Get trunk `cargo install --git https://github.com/thedodd/trunk.git trunk`. Note that it is necessary to get trunk from master because the current binary release has a bug in the proxy code that prevents delivering large images in multipart/form-data
4. Add WASM target `rustup target add wasm32-unknown-unknown`
5. Run the server: `cargo run --bin plant-monitor-server`
6. Run the client: `cd client && trunk serve --open`


## Credits:
Some parts of the code are borrowed from other open-source projects. See below.

* DateTime serialization: https://serde.rs/custom-date-format.html
* Stylesheets for buttons and top bar: https://www.w3schools.com/css/default.asp 