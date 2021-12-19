
#[macro_use] extern crate log;
extern crate geohash;
extern crate warp;

use std::error::Error;


//use geohash::{decode, encode, neighbor, Coordinate, Direction};
//use warp::Filter;
mod app;
use app::App;
 
fn main() -> Result<(), Box<dyn Error>> {
    //env_logger::init();

    App::new()
       .init();

    //info!("Yahoo");
    //println!("Hello World!");

    // encode a coordinate
    //let c = Coordinate { x: 112.5584f64, y: 37.8324f64 };
    //println!("encoding 37.8324, 112.5584: {}", encode(c, 10usize)?);
    // decode a geohash
    //let (c, lon_error, _) = decode("ww8p1r4t8y")?;
    //println!("decoding ww8p1r4t8y to: {}, {}", c.y, c.x);
    //println!("Long Error {}", lon_error);
    // find a neighboring hash
    //let sw = neighbor("ww8p1r4t8y", Direction::SW)?;
    Ok(())
}
