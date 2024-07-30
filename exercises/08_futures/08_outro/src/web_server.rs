use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tokio::task::JoinSet;

use crate::data::{Status, TicketDraft};
use crate::description::TicketDescription;
use crate::store::TicketStore;
use crate::title::TicketTitle;

pub async fn server(listener: TcpListener) {
    println!("SERVER: Starting up server");

    let mut store = TicketStore::new();
    let mut join_set = JoinSet::new();


    let store_arc = Arc::new(RwLock::new(store));
    loop {
        println!("SERVER: Accepting request");
        let (stream, _socket) = listener.accept().await.unwrap();
        let _ = join_set.spawn(process_req(store_arc.clone(), stream));
    }
}

async fn process_req(mut store: Arc<RwLock<TicketStore>>, mut stream: TcpStream) {
    println!("SERVER: Received request");
    let (mut reader, mut writer) = stream.split();
    let mut req = String::new();
    reader.read_to_string(&mut req).await.unwrap();
    println!("SERVER: Request content is: {req}");
    let mut split = req.split("\n");
    let command = split.next().unwrap();
    let data = split.next().unwrap();

    println!("SERVER: Request command is: {command}");
    println!("SERVER: Request data is: {data}");
    let resp = match command {
        "POST" => {
            let result = json::parse(data).unwrap();
            let draft = TicketDraft {
                title: TicketTitle::try_from(result["title"].as_str().unwrap()).unwrap(),
                description: TicketDescription::try_from(result["description"].as_str().unwrap()).unwrap(),
            };
            let ticket_id = store.write().await.add_ticket(draft);

            json::stringify(json::object! {
                    "id": u64::from(&ticket_id)
                })
        }
        "GET" => {
            let result = json::parse(data).unwrap();
            let id = result["id"].as_u64().unwrap();
            let ticket_opt = store.read().await.get(id.into());
            if let Some(ticket) = ticket_opt {
                let ticket_guard = &ticket.read().unwrap();
                json::stringify(json::object! {
                    "id": u64::from(&ticket_guard.id),
                    "title": String::from(&ticket_guard.title),
                    "description": String::from(&ticket_guard.description),
                    "status": String::try_from(&ticket_guard.status).unwrap()
                })
            } else {
                format!("No ticket with id {id}")
            }
        }
        "PATCH" => {
            let result = json::parse(data).unwrap();
            let id = result["id"].as_u64().unwrap();
            let ticket_opt = store.read().await.get(id.into());
            if let Some(ticket) = ticket_opt {
                {
                    let mut write_guard = ticket.write().unwrap();
                    if let Some(description) = result["description"].as_str() {
                        write_guard.description = TicketDescription::try_from(description).unwrap();
                    }
                    if let Some(title) = result["title"].as_str() {
                        write_guard.title = TicketTitle::try_from(title).unwrap();
                    }
                    if let Some(status_str) = result["status"].as_str() {
                        let status = Status::try_from(status_str).unwrap();
                        write_guard.status = status;
                    }
                }
                let read_guard = ticket.read().unwrap();

                json::stringify(json::object! {
                    "id": u64::from(&read_guard.id),
                    "title": String::from(&read_guard.title),
                    "description": String::from(&read_guard.description),
                    "status": String::try_from(&read_guard.status).unwrap()
                })
                // format!("Not ready")
            } else {
                format!("No ticket with id {id}")
            }
        }
        _ => {
            println!("SERVER: Unrecognized command: {command}");
            format!("Unrecognized command: {command}")
        }
    };

    writer.write_all(resp.as_bytes()).await.unwrap();
    writer.shutdown().await.unwrap();
}
