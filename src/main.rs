#[macro_use]
extern crate log;
extern crate chrono;
extern crate warp;

use chrono::Local;
use std::error::Error;
use std::io::Write;

mod kv;
mod routes;
mod handlers;
mod models;
use crate::kv::KVStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //configure logging
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}: [{}] - {}",
                Local::now(),
                record.level(),
                record.args()
            )
        })
        .init();

    info!("Yahoo!!!");

    //let db:kv::RocksDB = kv::KVStore::init("/tmp/rocksdb");
    let db: kv::RocksDB = kv::KVStore::init("/tmp/rocksdb");

    db.insert("one", "one");

    match db.lookup("one") {
        Some(v) => info!("Value: {}", v),
        None => info!("Value not found for key"),
    }

    match db.lookup("two") {
        Some(v) => info!("Value: {}", v),
        None => info!("Value not found for key"),
    }

    let customer_routes = routes::http_routes(db);
    warp::serve(customer_routes)
        .run(([127, 0, 0, 1], 3000))
        .await;

    Ok(())
}
