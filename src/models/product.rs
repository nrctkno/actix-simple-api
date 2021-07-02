
pub enum ProductStatus {
    Available,
    NotAvailable,
}

/*
In rust we do not create objects itself, we call them instances.
To create a instance you just use the struct name and a pair of {}, inside you put the name of the fields with values.
*/
pub struct Product {
    id: u32, //unsigned int
    name: String,
    price: u32,
    stock: u32,
    status: ProductStatus,
}

impl Product {
    /*
    creates a product
    */
    /*
    fn create(name: String, price: u32, stock: u32, status: ProductStatus) -> Product {
        //you can use return [whatever]; or simply avoid the ";" and remove the "return"
        Product {
            id: 1,
            name,
            price,
            stock,
            status,
        }
    }
    */

    /**
    saves a product and return the same product with the assigned id
    */
    /*
    fn save(mut p: Product) -> Product {
        p.id = 1;
        p //returns the updated product
    }
    */

    /**
    find products by name
    */
    fn find(name: String, page: u32, length: u32) {
/*        
        let stmt = conn
            .prepare("SELECT * FROM product WHERE name like '%%' ORDER BY name DESC LIMIT 10,1")?;

        stmt.query_map(NO_PARAMS, |row| {
            Ok(WeatherAgg::AnnualAgg {
                year: row.get(0)?,
                total: row.get(1)?,
            })
        })
        .and_then(Iterator::collect)
        */
    }
}
