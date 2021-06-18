use std::error::Error;
use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::fs::File;

pub struct Server {

}

pub fn run() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("Server listening!");

    for stream in listener.incoming() {
        handle_client(&mut stream?)?;
    }

    Ok(())
}

fn handle_client(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let mut bytes = [0_u8; 8];
    stream.read_exact(&mut bytes).expect("Failed to read filename size");
    let size = usize::from_be_bytes(bytes);
    println!("Received filename size: {}", size);

    let mut bytes = vec![0_u8; size];
    stream.read_exact(&mut bytes).expect("Failed to read filename");

    let name = String::from_utf8(bytes).expect("Failed to parse filename");
    println!("Received filename: {:?}", name);

    
    let mut bytes = [0_u8; 8];
    stream.read_exact(&mut bytes).expect("Failed to read file size");
    let size = usize::from_be_bytes(bytes);
    println!("Received file size: {}", size);
    
    let mut file = File::create(format!("./shared/{}", name))?;
    let mut bytes = [0_u8; 1024];
    let mut count = 0;
    let mut read = stream.read(&mut bytes).unwrap_or_default();
    while read == 1024 {
        let written = file.write(&mut bytes).expect("Failed to write to file");
        count += written;
        print!("Received: {:.2}\r", count as f32 / size as f32 * 100_f32);
        read = stream.read(&mut bytes).unwrap_or_default()
    }
    let written = file.write(&mut bytes[0..read]).expect("Failed to write to file");
    count += written;
    println!("Received: {:.2}", count as f32 / size as f32 * 100_f32);

    if count != size {
        Err(format!("Failed to write file completely: expected {} bytes but {} were written", size, count).into())
    } else {
        Ok(())
    }
}