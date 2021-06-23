use pancurses::{
    self,
    Window,
    Input
};
use std::sync::mpsc::{
    Receiver,
    Sender
};
use super::{ThreadState,Message};

pub fn run(sender: Sender<Message>) {
    let wnd = build_window();
    configure_pancurses();
    info!("Window initialized and configured");

    loop {
        if should_quit(wnd.getch()) {
            break sender.send(Message::Terminate).expect("Failed to send terminate message");
        }
    }

    info!("Shutting down UI thread");
    pancurses::endwin();
}

fn build_window() -> Window {
    let wnd = pancurses::initscr();
    wnd.keypad(true);
    wnd.nodelay(true);
    wnd
}

fn configure_pancurses() {
    pancurses::noecho();
    pancurses::resize_term(0, 0);
}

fn should_quit(ch: Option<Input>) -> bool {
    if let Some(Input::Character(c)) = ch {
        c == 'q' || c == '\u{1b}'
    } else {
        false
    }
}