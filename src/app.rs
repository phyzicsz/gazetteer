
extern crate chrono;

use chrono::prelude::*;
use std::io::Write;

pub struct App {}

impl App {
    pub fn new() -> App {
        App {}
    }

    pub fn init(self) -> App {
        
        //configure logging
        env_logger::builder()
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{}: [{}] - {}",
                    Utc::now(),
                    record.level(),
                    record.args()
                )
            })
            .init();

            info!("Yahoo!!!");

        self
    }
}
