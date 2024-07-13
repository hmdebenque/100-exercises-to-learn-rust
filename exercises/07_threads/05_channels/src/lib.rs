use std::sync::mpsc::{Receiver, Sender};
use std::thread::spawn;
use crate::data::TicketDraft;
use crate::store::TicketStore;

pub mod data;
pub mod store;

pub enum Command {
    Insert(TicketDraft),
}

// Start the system by spawning the server the thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    spawn(move || server(receiver));
    sender
}

// TODO: The server task should **never** stop.
//  Enter a loop: wait for a command to show up in
//  the channel, then execute it, then start waiting
//  for the next command.
pub fn server(receiver: Receiver<Command>) {
    spawn(move || {
        let mut store = TicketStore::new();
        loop {
            let result = receiver.recv();
            if result.is_err() {
                break;
            }
            match result.unwrap() {
                Command::Insert(ticket_draft) => {
                    store.add_ticket(ticket_draft);
                }
            }
            println!("Store state is {store:?}");
        }
    });
}
