use actix_web::{HttpResponse, Responder};

pub async fn get() -> impl Responder {
    HttpResponse::Ok().body("Products API\n List products: GET /products?page=1&length=10\n Create a product: POST /products\n Sell a product: POST /sales/{sku}\n")
}
