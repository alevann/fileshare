use std::sync::{
    mpsc::{Sender, Receiver}
};
use std::net::{TcpListener, TcpStream, SocketAddr};
use super::{ThreadState, Message};
use crate::thread_pool::ThreadPool;
use std::io::{Read,Write};
use std::fs::File;


pub fn run(receiver: Receiver<Message>, sender: Sender<ThreadState>) {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    let pool = ThreadPool::new(2);

    info!("Server pool started and listener bound");

    loop {
        if let Ok(Message::Terminate) = receiver.try_recv() {
            break info!("Server shutdown message received");
        }
        if let Ok(connection) = listener.accept() {
            let sender = sender.clone();
            pool.execute(|worker_id| {
                handle_connection(worker_id, connection, sender)
            })
        }
    }
}

fn handle_connection(
    worker_id: usize,
    (stream, addr): (TcpStream, SocketAddr),
    sender: Sender<ThreadState>
) {
    info!("Received a new connection: {}!", addr);
    let mut stream = StreamWrapper { stream };

    let name = stream.read_usize();
    let name = stream.read_string(name);

    let size = stream.read_usize();
    let mut file = if let Ok(file) = File::create(format!("./shared/{}", name)) {
        file
    } else {
        return error!("Failed to create file at `./shared/{}'", name)
    };

    let build_state = |count, size| {
        ThreadState {
            thread: worker_id,
            complete: count as f32 / size as f32 * 100_f32
        }
    };
    let mut is_receiver_connected = true;

    let mut buffer = [0_u8; 1024];
    let mut written_count = 0;
    while stream.read(&mut buffer) > 0 {
        match file.write(&mut buffer) {
            Ok(count) => {
                written_count += count;
                if is_receiver_connected {
                    is_receiver_connected = sender.send(build_state(written_count, size)).is_ok();
                    if !is_receiver_connected {
                        warn!("Receiver is disconnected: worker state won't be sent anymore (worker id: {})", worker_id);
                    }
                }
            }
            Err(err) => {
                drop(file);
                handle_file_write_error(err, written_count, &name);
                return;
            }
        };
    }
}

fn handle_file_write_error(err: std::io::Error, written_count: usize, name: &String) {
    error!("Failed to write to file after writing {}b (file: {})", written_count, name);
    error!("Fail to write to file was caused by: {}", err);
    if let Err(e) = std::fs::remove_file(format!("./shared/{}", name)) {
        error!("Failed to delete file: {}", e);
    } else {
        error!("File deleted has been deleted, freeing worker thread");
    }
}

struct StreamWrapper {
    stream: TcpStream
}

impl StreamWrapper {
    pub fn read_usize(&mut self) -> usize {
        let mut buffer = [0_u8; 8];
        self.stream.read_exact(&mut buffer).unwrap();
        usize::from_be_bytes(buffer)
    }
    pub fn read_string(&mut self, size: usize) -> String {
        let mut buffer = vec![0_u8; size];
        self.stream.read_exact(&mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }
    pub fn read(&mut self, buffer: &mut [u8]) -> usize {
        self.stream.read(buffer).unwrap_or_default()
    }
}

