use std::net::TcpStream;
use std::io::{Write,Read};
use std::fs::{File};
use std::path::Path;
use std::time::Instant;
use super::Config;
use crate::thread_pool::ThreadPool;


pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let pool = ThreadPool::new(2);

    for file in config.file_list.clone() {
        if !Path::new(&file).exists() {
            warn!("File {} does not exists, skipping", file);
            continue
        }
        
        let endpoint = config.connect_to.clone();
        pool.execute(move |_| {
            send_file(endpoint, file.clone())
        })
    }

    Ok(())
}

fn send_file(endpoint: String, path: String) {
    let mut stream = match TcpStream::connect(endpoint.clone()) {
        Ok(stream) => stream,
        Err(e) => return error!("Could not connect to {}: {}", endpoint, e)
    };
    
    let filename = path.split("/").last()
        .expect("Failed to get filename")
        .to_string();
    let mut filehandle = match File::open(path) {
        Ok(handle) => handle,
        Err(e) => return error!("Failed to open file {}: {}", filename, e)
    };
    let filesize = filehandle.metadata()
        .expect("Failed to read file metadata").len();

    debug!("Sharing: {} ({}b)", filename, filesize);


    let bytes = filename.as_bytes();
    let bytes: [u8; 8] = unsafe { std::mem::transmute(bytes.len().to_be()) };    
    stream.write(&bytes).expect("Failed to write filename size");
    stream.write(filename.as_bytes()).expect("Failed to write filename");
    
    let bytes: [u8; 8] = unsafe { std::mem::transmute(filesize.to_be()) };
    stream.write(&bytes).expect("Failed to write filesize");

    let instant = Instant::now();
    let elapsed = || { instant.elapsed().as_secs() };

    let mut read_count = 0;
    let mut written_count = 0;
    let mut buffer = [0_u8; 1024];
    let mut last_read = filehandle.read(&mut buffer).unwrap_or_default();
    while last_read > 0 {
        let write_res = stream.write(&buffer[0..last_read]);
        if write_res.is_err() {
            error!("Failed to write to stream ({}b out of {}b written)", written_count, filesize);
        } else {
            written_count += write_res.unwrap();
        }
        read_count += last_read;
        last_read = filehandle.read(&mut buffer).unwrap_or_default();
    }

    info!("File shared in {}s", elapsed());
    if written_count > read_count {
        error!("Possible corruption: written {}b but {}b were read", written_count, read_count);
    }
    if read_count as u64 != filesize {
        error!("Possible corruption: {}b were read when filesize is {}b", read_count, filesize);
    }
    if filesize != written_count as u64 {
        error!("Possible corruption: written {}b but filesize is {}b", written_count, filesize);
    }
}