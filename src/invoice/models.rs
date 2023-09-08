use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateEntryData {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub quantity: i32,
}

#[derive(Deserialize, Clone)]
pub struct CreateEntryDataCart {
    pub id: String,
    pub quantity: i32,
}

#[derive(Deserialize, Clone)]
pub struct UpdateEntryDataCartQty {
    //pub id: String,
    pub qty: i32,
}
