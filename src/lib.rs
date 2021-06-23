#[macro_use]
extern crate log;
extern crate pancurses;

mod server;
mod client;
mod tserv;
mod cli;
mod thread_pool;

use std::error::Error;
use std::sync::mpsc;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    debug!("{:?}", config);

    let (sender, receiver) = mpsc::channel();
    let ui_thread = std::thread::spawn(move || {
        cli::run(sender)
    });
    let sr_thread = std::thread::spawn(move || {
        server::run(receiver)
    });

    if let Err(_) = ui_thread.join() {
        return Err("Failed to shutdown UI thread".into())
    }
    if let Err(_) = sr_thread.join() {
        Err("Failed to shutdown server thread".into())
    } else {
        Ok(())
    }
}


#[derive(Debug)]
pub struct Config {
    as_server: bool,
    file_list: Vec<String>,
    connect_to: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() == 0 {
            Ok(Config {
                as_server: true,
                file_list: Vec::new(),
                connect_to: String::new()
            })
        } else {
            Ok(Config {
                as_server: false,
                file_list: args[1..].to_vec(),
                connect_to: args[0].clone()
            })
        }
    }
}


pub struct ThreadState {
    thread: u64,
    percent: f32,
    file: String,
}

pub enum Message {
    Terminate
}