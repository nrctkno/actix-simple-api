use super::models::product::{NewProduct, Product, ProductStatus};
use super::schema::products::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse, Responder};
use diesel::dsl::insert_into;
//use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::vec::Vec;


#[derive(Debug, Serialize, Deserialize)]
pub struct InputProduct {
    name: String,
//    price: i32,
    stock: i32,
    status: ProductStatus,
}

// Handler for GET /
pub async fn home() -> impl Responder {
    HttpResponse::Ok().body("<!doctype html><html><body><h1>Products API</h1><br />List products: GET <a href=\"/api/v1/products?page=1&length=10\">/products</a><br /> Create a product: POST /products<br />Sell a product: POST /sales/{sku}</body></html>")
}

// Handler for GET /products
pub async fn get(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || find_products(db))
        .await
        .map(|a_product| HttpResponse::Ok().json(a_product))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for POST /products
pub async fn post(
    db: web::Data<Pool>,
    item: web::Json<InputProduct>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_product(db, item))
        .await
        .map(|a_product| HttpResponse::Created().json(a_product))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

/*

"Repository" section

*/
fn find_products(pool: web::Data<Pool>) -> Result<Vec<Product>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = products.load::<Product>(&conn)?;
    Ok(items)
    //HttpResponse::Ok().body("list of products goes here")
}

fn add_product(
    db: web::Data<Pool>,
    item: web::Json<InputProduct>,
) -> Result<Product, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_product = NewProduct {
        name: &item.name,
        //        price: &item.price,
        stock: &item.stock,
    };
    //let res = insert_into(products)
    insert_into(products)
        .values(&new_product)
        //.get_result(&conn)?;
        .execute(&conn)
        .expect("Error");
    //let res = products.order(id.desc()).first(&conn).unwrap();
    let res = products.find(id).get_result::<Product>(&conn).unwrap();
    Ok(res)
}
