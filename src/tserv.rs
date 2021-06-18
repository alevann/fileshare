use std::error::Error;
use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::sync::mpsc::{self,Sender,Receiver};
use std::thread::{ThreadId,self};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use pancurses::{Window,Input};

struct ThreadState
{
    thread: ThreadId,
    file: String,
    percent: f32
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    let (tx, rx) = mpsc::channel();

    start_ui_thread(rx);

    for stream in listener.incoming() {
        handle_threaded(tx.clone(), stream?);
    }

    Ok(())
}

fn handle_threaded(tx: Sender<ThreadState>, mut stream: TcpStream) {
    std::thread::spawn(move || {
        handle_client(tx.clone(), &mut stream);
    });
}

fn handle_client(tx: Sender<ThreadState>, stream: &mut TcpStream) {
    let read_size = |stream: &mut TcpStream| {
        let mut bytes = [0_u8; 8];
        stream.read_exact(&mut bytes).expect("Failed to read size");
        usize::from_be_bytes(bytes)
    };

    let send_state = |file, percent| {
        tx.send(ThreadState {
            file,
            percent,
            thread: thread::current().id()
        }).expect("Could not update thread state")
    };

    let size = read_size(stream);
    let mut name = vec![0_u8; size];
    stream.read_exact(&mut name).expect("Failed to read filename");
    let name = String::from_utf8(name).expect("Failed to parse filename");
    let mut name = name.split("\\").last().expect(":(").to_string();

    while Path::new(&format!("./shared/{}", name)).exists() {
        name += "(copy)";
    }
    
    let size = read_size(stream);
    let mut file = File::create(format!("./shared/{}", name)).expect("Failed to create file");
    let mut bytes = [0_u8; 1024];
    let mut count = 0;
    let mut read = stream.read(&mut bytes).unwrap_or_default();
    while read == 1024 {
        let written = file.write(&mut bytes).expect("Failed to write to file");
        count += written;
        send_state(name.clone(), count as f32 / size as f32 * 100_f32);
        read = stream.read(&mut bytes).unwrap_or_default()
    }
    let written = file.write(&mut bytes[0..read]).expect("Failed to write to file");
    count += written;
    send_state(name.clone(), count as f32 / size as f32 * 100_f32);
}

fn start_ui_thread(rx: Receiver<ThreadState>) {
    std::thread::spawn(move || {
        let mut map = HashMap::new();
        let wnd = pancurses::initscr();
        wnd.keypad(true);
        wnd.nodelay(true);
        pancurses::noecho();
        pancurses::resize_term(0, 0);

        display_thread_state(&map, &wnd);

        loop {
            match wnd.getch() {
                Some(Input::KeyDC) => break,
                Some(Input::KeyExit) => break,
                Some(Input::KeyClose) => break,
                Some(Input::KeyResize) => { pancurses::resize_term(0, 0); },
                _ => ()
            }
            while let Ok(msg) = rx.try_recv() {
                map.insert(msg.thread, msg);
            }
            display_thread_state(&map, &wnd);
        }

        pancurses::endwin();
        std::process::exit(0)
    });
}

fn display_thread_state<K: std::fmt::Debug>(map: &HashMap<K, ThreadState>, wnd: &Window) {
    wnd.clear();
    wnd.addstr(String::from("Current state:\n\n"));
    for (k, v) in map.iter() {
        wnd.addstr(format!("[{:?}]\t{:.2}%\t[{}{}]\t{}\n", k, v.percent, String::from("=").repeat(v.percent as usize), String::from("-").repeat(100-v.percent as usize), v.file));
    }
    wnd.refresh();
}
// 192.168.1.51