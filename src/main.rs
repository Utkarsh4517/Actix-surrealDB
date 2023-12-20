use actix_web::{get, post, patch, App, HttpServer, Responder, HttpResponse};

// endpoints
#[get("/pizzas")] 
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas available!")
}

#[post("/buypizzas")]
async fn buy_pizzas() -> impl Responder {
    HttpResponse::Ok().body("buying a pizza")
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza() -> impl Responder {
    HttpResponse::Ok().body("updating a pizza")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(get_pizzas).service(buy_pizzas).service(update_pizza)
    }).bind("127.0.0.1:8080")?.run().await
}
