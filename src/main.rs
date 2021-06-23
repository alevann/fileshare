#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;
use std::env;
use std::fs::File;
use fileshare::Config;


macro_rules! exit {
    ( $($arg:tt),* ) => {
        eprintln!($($arg,)*);
        std::process::exit(1);
    };
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args[1..]).unwrap_or_else(|e| {
        exit!("{}", e);
    });

    if let Err(e) = init_logging() {
        exit!("Log initialization error: {}", e);
    } else {
        info!("Logging initialized successfully!")
    }

    if let Err(e) = fileshare::run(config) {
        exit!("Application error: {}", e);
    }
}

fn init_logging() -> Result<(), &'static str> {
    let file = if let Ok(file) = File::create("./logs/log.log") {
        file
    } else {
        return Err("Failed to create logfile")
    };

    if let Err(_) = WriteLogger::init(
        LevelFilter::Trace,
        simplelog::Config::default(),
        file
    ) {
        Err("Failed to initialize logger")
    } else {
        Ok(())
    }
}
