use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod invoice;
use invoice::services;

//-----------------------Stock-------------------------------
struct StockItems {
    stock_items: Mutex<HashMap<String, ItemDetails>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct ItemDetails {
    name: String,
    price: f64,
    quantity: i32,
}

//------------------------cart-------------------------
struct Invoice {
    invoice_list: Mutex<HashMap<String, InvoiceDetails>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct InvoiceDetails {
    name: String,
    price: f64,
    quantity: i32,
    total: f64,
}

//----------------------------------------------------

#[get("/")]
async fn index() -> String {
    "Thuis is a helth check".to_string()
}

#[get("/feature")]
async fn value() -> String {
    "featuring to test".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //new instance of web::Data is created, Inside web::Data, there's an instance of AppState being created
    let app_data = web::Data::new(StockItems {
        stock_items: Mutex::new(HashMap::new()),
    });

    let app_data_cart = web::Data::new(Invoice {
        invoice_list: Mutex::new(HashMap::new()),
    });

    println!("Server Started");

    // initializes a new instance of an Actix HTTP server
    HttpServer::new(move || {
        App::new() //This creates a new Actix web application. An Actix web application is a collection of routes and middleware that handle incoming HTTP requests
            .app_data(app_data.clone())
            .app_data(app_data_cart.clone())
            .service(index)
            .service(value) //call indexx function -> helth check
            .configure(services::config) //call services
    })
    .bind(("127.0.0.1", 8080))?
    .run() //This method starts the Actix web server and makes it listen for incoming HTTP requests on the specified address and port.
    .await
}
