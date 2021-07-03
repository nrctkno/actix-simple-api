use crate::actions::product::InputProduct;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::product::{NewProduct, Product};
use crate::schema::products::dsl::*;
use crate::Pool;
use actix_web::web;
use diesel::dsl::insert_into;
use std::vec::Vec;

pub fn find_products(pool: web::Data<Pool>) -> Result<Vec<Product>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = products.load::<Product>(&conn)?;
    Ok(items)
    //HttpResponse::Ok().body("list of products goes here")
}

pub fn add_product(
    db: web::Data<Pool>,
    item: web::Json<InputProduct>,
) -> Result<Product, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_product = NewProduct {
        name: &item.name,
        price: &item.price,
        stock: &item.stock,
        status: &item.status,
    };
    //let inserted = insert_into(products)
    
      insert_into(products)
          .values(&new_product)
          //.get_result(&conn)?;
          .execute(&conn)?; //? is similar to append .expect("Error")

      //let id = (products.count().get_result(&conn).unwrap_or(0)) as i32;
      let inserted = products.find(id).get_result::<Product>(&conn).unwrap();
      Ok(inserted)
    
}
