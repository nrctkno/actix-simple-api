use crate::schema::*;
use serde::{Deserialize, Serialize};

/*
In rust we do not create objects itself, we call them instances.
To create a instance you just use the struct name and a pair of {}, inside you put the name of the fields with values.
Product is used while reading a product from the table.
*/
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Product {
    id: i32, //unsigned int
    name: String,
    price: i32,
    stock: i32,
    status: String,
}

/*
NewProduct is used while inserting a user to the users table.
*/
#[derive(Insertable, Debug)]
#[table_name = "products"]
pub struct NewProduct<'a> {
    pub name: &'a str,
    pub price: &'a i32,
    pub stock: &'a i32,
    pub status: &'a str,
}
