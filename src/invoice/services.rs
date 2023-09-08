use super::models::{CreateEntryData, CreateEntryDataCart, UpdateEntryDataCartQty};
use crate::{Invoice, InvoiceDetails, ItemDetails, StockItems};
use actix_web::{
    delete, get, post, put,
    web::{self, service},
    HttpResponse, Responder,
};
use serde_json::json;

#[get("/feature")]
async fn value() -> String {
    "featuring to test".to_string()
}

#[post("/invoice/entries")]
async fn create_entry(
    data: web::Data<StockItems>,
    param_obj: web::Json<CreateEntryData>,
) -> impl Responder {
    //println!("Server check9");
    let mut stock_items = data.stock_items.lock().unwrap();

    stock_items.insert(
        param_obj.id.clone(),
        ItemDetails {
            name: param_obj.name.clone(),
            price: param_obj.price,
            quantity: param_obj.quantity,
        },
    );

    println!("Server check");

    HttpResponse::Ok().json(&*stock_items)
}

#[get("/invoice/entries")]
async fn get_items(data: web::Data<StockItems>) -> impl Responder {
    let mut stock_items = data.stock_items.lock().unwrap();
    HttpResponse::Ok().json(&*stock_items)
}

#[post("/invoice/cart")]
async fn create_entry_cart(
    data: web::Data<Invoice>,
    cart: web::Data<StockItems>,
    param_obj: web::Json<CreateEntryDataCart>,
) -> impl Responder {
    let mut invoice_list = data.invoice_list.lock().unwrap();
    let mut stock_items = cart.stock_items.lock().unwrap();

    match stock_items.get(&param_obj.id) {
        Some(stockValue) => {
            if param_obj.quantity >= stockValue.quantity {
                HttpResponse::BadRequest().body("Doesn;t have reqeusted quantity")
            } else {
                invoice_list.insert(
                    param_obj.id.clone(),
                    InvoiceDetails {
                        name: stockValue.name.clone(),
                        price: stockValue.price,
                        quantity: param_obj.quantity,
                        total: stockValue.price * param_obj.quantity as f64,
                    },
                );
                HttpResponse::Ok().json(&*invoice_list)
            }
            //let mut tot_price = 0;
            //HttpResponse::BadRequest().body("{:?}" : tot_ptice)
        }
        None => HttpResponse::BadRequest().body("Doesn;t have reqeusted Item in the Stock"),
    }
}

#[put("/invoice/cart/{id}")]
async fn update_entry_cart(
    data: web::Data<Invoice>,
    path: web::Path<String>,
    param_obj: web::Json<UpdateEntryDataCartQty>,
) -> impl Responder {
    let mut invoice_list = data.invoice_list.lock().unwrap();
    let mut id = path.into_inner();

    match invoice_list.get_mut(&id) {
        Some(invoiceValue) => {
            invoiceValue.quantity += param_obj.qty;
            invoiceValue.total = invoiceValue.quantity as f64 * invoiceValue.price;
        }
        None => (), //=> HttpResponse::BadRequest().body("Doesn;tH have reqeusted Item in the Stock"),
    }

    HttpResponse::Ok().json(&*invoice_list)
}

#[get("/invoice/cart_getall")]
async fn get_allcart_items(data: web::Data<Invoice>) -> impl Responder {
    let mut invoice_list = data.invoice_list.lock().unwrap();

    let full_total: f64 = invoice_list
        .values()
        .map(|invoiceValue| invoiceValue.total)
        .sum();

    HttpResponse::Ok().json(json!({
        "invoice_list": &*invoice_list,
        "full_total": full_total,
    }))
}

// #[get("/invoice/cart_getall")]
// async fn get_cart(data: web::Data<Invoice>) -> impl Responder {
//     let mut invoice_list = data.invoice_list.lock().unwrap();
//     match invoice_list.get_key_value(k{
//         Some(invoiceValue : &InvoiceDetails) =>{

//         }
//         None
//     })

//     //HttpResponse::Ok().json(&*invoice_list)
// }

//pass service function on to the web server
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(value)
        .service(create_entry)
        .service(get_items)
        .service(create_entry_cart)
        .service(update_entry_cart)
        .service(get_allcart_items);
}

/*

#[get("/invoice/entries")]
async fn get_items(data: web::Data<StockItems>) -> impl Responder {
    //returns a type that implements the Responder trait. This trait defines how a value can be converted into an HTTP response.
    HttpResponse::Ok().json(data.stock_items.lock().unwrap().values())
}

#[post("/invoice/entries")]
async fn create_item(
    data: web::Data<StockItems>,
    param_obj: web::Json<CreateEntryData>,
) -> impl Responder {
    let mut stock_items = data.stock_items.lock().unwrap();
    let mut max_id: i32 = 0;

    for i in 0..stock_items.len() {
        if stock_items.keys() > max_id {
            max_id = stock_items.keys();
        }
    }

    stock_items.insert(ItemDetails {
        id: max_id + 1,
        title: param_obj.title.clone(),
        date: param_obj.date,
    });

    HttpResponse::Ok().json(todolist_entries.to_vec())
}
async fn add_grocery_list_item(
    item: Item,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let r = store.grocery_list.read();
    Ok(warp::reply::json(&*r))
}

fn add(&mut self, bill: Bill) {
    self.inner.insert(bill.name.to_string(), bill);
}

#[post("/invoice/entries")]
async fn create_item(
    item: ItemDetails,
    store: StockItems,
) -> Result<impl warp::Reply, warp::Rejection> {
    let r = store.stock_items.read();
    Ok(warp::reply::json(&*r))
}

*/
