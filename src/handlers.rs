use std::convert::Infallible;

use warp::{self, http::StatusCode};

use crate::kv::RocksDB;
use crate::models::Customer;

/// Returns a list of customers as JSON
///
/// # Arguments
///
/// * `db` - `Db` -> thread safe vector of Customer objects
pub async fn list_customers(db: RocksDB) -> Result<impl warp::Reply, Infallible> {
    //let customers = db.lock().await;
    //let customers: Vec<Customer> = customers.clone();
    let customer = Customer {
        guid: String::from("123"),
        first_name: String::from("jim"),
        last_name: String::from("bob"),
        email: String::from("jim.bob@gmail.com"),
        address: String::from("123 woodford street")
    };

    Ok(warp::reply::json(&customer))
}
