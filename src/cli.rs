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
use std::collections::HashMap;

pub fn run(sender: Sender<Message>, receiver: Receiver<ThreadState>) {
    let wnd = build_window();
    configure_pancurses();
    info!("Window initialized and configured");

    let mut thread_state_map = HashMap::new();
    loop {
        if should_quit(wnd.getch()) {
            break sender.send(Message::Terminate).expect("Failed to send terminate message")
        }
        while let Ok(state) = receiver.try_recv() {
            update_state(&mut thread_state_map, state);
        }
        display_state(&wnd, &thread_state_map);
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

fn update_state(global_state: &mut HashMap<usize, u8>, state: ThreadState) {
    global_state.insert(state.thread, state.complete);
}

fn display_state(wnd: &Window, state: &HashMap<usize, u8>) {
    wnd.clear();
    wnd.addstr("Current server pool state:\n\n");

    let eq = String::from("=");
    let ne = String::from("-");

    for (k, v) in state.iter() {
        wnd.addstr(format!(
            "[{}]\t{:.2}%\t[{}{}]\n",
            k,
            v,
            eq.repeat(*v as usize),
            ne.repeat((100-v) as usize)
        ));
    }
    
    wnd.refresh();
}