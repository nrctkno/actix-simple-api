use actix_web::{HttpResponse, Responder};

pub async fn post(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn get(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("list of products goes here")
}
