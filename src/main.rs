use std::process;
use std::env;
use fileshare::Config;

macro_rules! e_exit {
    () => {
        process::exit(1)
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args[1..]).unwrap_or_else( |e| {
        eprintln!("{}", e);
        e_exit!()
    });

    if let Err(e) = fileshare::run(config) {
        eprintln!("Application error: {}", e);
        e_exit!()
    }
}
