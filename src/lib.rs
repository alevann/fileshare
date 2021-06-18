extern crate pancurses;

mod server;
mod client;
mod tserv;

use std::error::Error;
use pancurses::Input;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{:#?}", config);

    if config.as_server {
        tserv::run()
    } else {
        client::run(config)
    }
}

#[derive(Debug)]
pub struct Config {
    as_server: bool,
    file_list: Option<Vec<String>>,
    connect_to: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() == 0 {
            Ok(Config {
                as_server: true,
                file_list: None,
                connect_to: String::new()
            })
        } else {
            Ok(Config {
                as_server: false,
                file_list: Some(args[1..].to_vec()),
                connect_to: args[0].clone()
            })
        }
    }
}

fn window() {
    let wnd = pancurses::initscr();
    wnd.printw("Type things, press delete to quit\n");
    wnd.refresh();
    wnd.keypad(true);
    
    pancurses::noecho();

    loop {
        match wnd.getch() {
            Some(Input::Character(c)) => { wnd.addch(c); },
            Some(Input::KeyDC) => break,
            Some(input) => { wnd.addstr(&format!("{:?}", input)); },
            None => ()
        }
    }

    pancurses::endwin();
}



