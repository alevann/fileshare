use std::sync::{
    mpsc::{Sender, Receiver},
    Arc,
    Mutex
};
use std::net::{TcpListener, TcpStream, SocketAddr};
use super::{ThreadState, Message};
use crate::thread_pool::ThreadPool;


pub fn run(receiver: Receiver<Message>) {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    listener.set_nonblocking(true)
        .expect("Failed to set non-blocking server socket");
    
    let pool = ThreadPool::new(4);
    info!("Server pool started");

    loop {
        if let Ok(Message::Terminate) = receiver.try_recv() {
            break info!("Server shutdown message received");
        }
        if let Ok(connection) = listener.accept() {
            pool.execute(|| { handle_connection(connection) })
        }
    }
}

fn handle_connection((stream, addr): (TcpStream, SocketAddr)) {
    info!("Received a new connection: {}", addr);
}

