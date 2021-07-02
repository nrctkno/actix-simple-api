use actix_web::{HttpResponse, Responder};

pub async fn get() -> impl Responder {
    HttpResponse::Ok().body("<!doctype html><html><body><h1>Products API</h1><br />List products: GET <a href=\"/api/v1/products?page=1&length=10\">/products</a><br /> Create a product: POST /products<br />Sell a product: POST /sales/{sku}</body></html>")
}
