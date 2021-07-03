use crate::repositories;
use crate::Pool;
use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputProduct {
    pub name: String,
    pub price: i32,
    pub stock: i32,
    pub status: String,
}

// Handler for GET /products
pub async fn get(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || repositories::product::find_products(db))
        .await
        .map(|a_product| HttpResponse::Ok().json(a_product))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for POST /products
pub async fn post(
    db: web::Data<Pool>,
    item: web::Json<InputProduct>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || repositories::product::add_product(db, item))
            .await
            .map(|a_product| HttpResponse::Created().json(a_product))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}
