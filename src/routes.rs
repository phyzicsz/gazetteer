use std::convert::Infallible;
use warp::{self, Filter};

use crate::handlers;
use crate::kv::RocksDB;

/// All routes
pub fn http_routes(
    db: RocksDB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    //get_customer(db.clone())
    //    .or(update_customer(db.clone()))
    //    .or(delete_customer(db.clone()))
    //    .or(create_customer(db.clone()))
    //    .or(customers_list(db))
    get_types(db.clone())
}

fn get_types(
    db: RocksDB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("types")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::list_customers)
}

fn with_db(db: RocksDB) -> impl Filter<Extract = (RocksDB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}