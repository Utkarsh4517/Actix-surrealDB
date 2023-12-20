use actix_web::web::Data;
use actix_web::{get, patch, post, web::Json, web::Path, App, HttpResponse, HttpServer, Responder};
mod db;
mod models;
use crate::db::Database;
use crate::models::pizza::{BuyPizzaRequest, UpdatePizzaURL};
use validator::Validate;

// endpoints

// Endpoint to get a a list of pizzas
#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas available!")
}

// Endpoint to buy a pizza
#[post("/buypizza")]
async fn buy_pizzas(body: Json<BuyPizzaRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            HttpResponse::Ok().body(format!("Pizza entered is {pizza_name}"))
        }
        Err(_) => HttpResponse::Ok().body("Pizza name is required"),
    }
}

// Endpoint to update a pizza
#[patch("/updatepizza/{uuid}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaURL>) -> impl Responder {
    let uuid = update_pizza_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("updating a pizza with {uuid}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init database
    let db = Database::init()
        .await
        .expect("error connecting to database");
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_pizzas)
            .service(buy_pizzas)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
