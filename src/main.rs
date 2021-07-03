mod handlers;
mod models;
mod schema;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

#[macro_use]
extern crate diesel;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*
    env vars
    */
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    /*
    create db connection pool
    */
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    /*
    Create server
    */
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(
                // prefixes all resources and routes attached to it...
                web::scope("/api/v1")
                    //.route("/products", web::get().to(products::get))
                    .route("/products", web::get().to(handlers::get))
                    //.route("/products", web::post().to(products::post))
                    .route("/products", web::post().to(handlers::post)), //.route("/products/{id}/sell", web::post().to(sales::post)),
            )
            //.route("/", web::get().to(home::get))
            .route("/", web::get().to(handlers::home))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
