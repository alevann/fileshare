use std::sync::{
    mpsc::{Sender, Receiver}
};
use std::net::{TcpListener, TcpStream, SocketAddr};
use super::{ThreadState, Message};
use crate::thread_pool::ThreadPool;
use rand::Rng;


pub fn run(receiver: Receiver<Message>, sender: Sender<ThreadState>) {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    listener.set_nonblocking(true)
        .expect("Failed to set non-blocking server socket");
    
    let pool = ThreadPool::new(15);
    info!("Server pool started");

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

    let mut rng = rand::thread_rng();

    for i in 0..101 {
        sender.send(ThreadState { thread: worker_id, complete: i }).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(rng.gen_range(100..600)))
    }
}

