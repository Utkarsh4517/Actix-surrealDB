use actix_web::web::Data;
use actix_web::{get, patch, post, web::Json, web::Path, App, HttpResponse, HttpServer, Responder};
mod db;
mod models;
use crate::db::Database;
use crate::errors::PizzaError;
use crate::models::pizza::{BuyPizzaRequest, Pizza, UpdatePizzaURL};
use uuid;
use validator::Validate;
mod errors;
// endpoints

// Endpoint to get a a list of pizzas
#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> Result<Json<Vec<Pizza>>, PizzaError> {
    let pizzas = db.get_all_pizzas().await;
    match pizzas {
        Some(found_pizzas) => Ok(Json(found_pizzas)),
        None => Err(PizzaError::NoPizzasFound),
    }
}

// Endpoint to buy a pizza
#[post("/buypizza")]
async fn buy_pizzas(
    body: Json<BuyPizzaRequest>,
    db: Data<Database>,
) -> Result<Json<Pizza>, PizzaError> {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();

            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);
            let new_pizza = db
                .add_pizza(Pizza::new(String::from(new_uuid), pizza_name))
                .await;

            match new_pizza {
                Some(created) => Ok(Json(created)),
                None => Err(PizzaError::PizzaCreationFailure),
            }
        }
        Err(_) => Err(PizzaError::PizzaCreationFailure),
    }
}

// Endpoint to update a pizza
#[patch("/updatepizza/{uuid}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaURL>, db: Data<Database>) -> Result<Json<Pizza>, PizzaError> {
    let uuid = update_pizza_url.into_inner().uuid;
    let update_result = db.update_pizza(uuid).await;
    match update_result {
        Some(updated_pizza) => Ok(Json(updated_pizza)),
        None => Err(PizzaError::NoSuchPizzasFound)
    }
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
