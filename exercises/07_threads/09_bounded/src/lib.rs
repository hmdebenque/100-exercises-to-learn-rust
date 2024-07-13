// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, sync_channel, SyncSender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, &str> {
        let (response_channel, response_receiver) = sync_channel(1);
        let command = Command::Insert { draft, response_channel };
        self.exchange(command, response_receiver)
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, &str> {
        let (response_channel, response_receiver) = sync_channel(1);
        let command = Command::Get { id, response_channel };
        self.exchange(command, response_receiver)
    }

    fn exchange<T>(&self, command: Command, response_receiver: Receiver<T>) -> Result<T, &str> {
        let result = self.sender.try_send(command);
        if result.is_err() {
            return Err("Error, channel is full.");
        } else {
            let response = response_receiver.recv();
            return if response.is_err() {
                Err("Couldn't get response")
            } else {
                Ok(response.unwrap())
            };
        }
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                   draft,
                   response_channel,
               }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                   id,
                   response_channel,
               }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
