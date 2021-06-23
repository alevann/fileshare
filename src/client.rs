use std::error::Error;
use std::net::TcpStream;
use std::io::{Read,Write};
use std::mem;
use std::fs::{File,self};
use std::path::Path;

use rand::Rng;

use super::Config;

pub struct Client {

}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let files = config.file_list;

    /*for file in files.clone() {
        if !Path::new(&file).exists() {
            eprintln!("File {} does not exists", file);
            std::process::exit(1);
        }
    }*/

    let mut threads = vec![];
    let mut rng = rand::thread_rng();

    for file in 0..rng.gen_range(1..13) {
        let connect_to = config.connect_to.clone();
        let thread = std::thread::spawn(move || {
            /*let fh = File::open(file.clone())
                .expect("Failed to open file");*/

            let mut stream = TcpStream::connect(connect_to)
                .expect("Could not open stream");

            /*let file_name: String = file.split("/").last()
                .expect("Failed to get filename")
                .to_string();

            let bytes = file_name.as_bytes();
            let size_as_bytes: [u8; 8] = unsafe { mem::transmute(bytes.len().to_be()) };

            println!("Sending size: {}", bytes.len());
            println!("Sending filename: {:?}", file_name);

            stream.write(&size_as_bytes).expect("Failed to write size as bytes");
            stream.write(bytes).expect("Failed to write filename bytes");

            let file_size = fh.metadata().expect("Failed to read file metadata").len();
            let size_as_bytes: [u8; 8] = unsafe { mem::transmute(file_size.to_be()) };
            
            println!("Sendfing file size: {}", file_size);

            stream.write(&size_as_bytes).expect("Failed to send file size");

            let content = fs::read(file.clone()).expect("Failed to read file");
            stream.write(&content).expect("Failed  to send file content");*/
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join();
    }
    
    Ok(())
}