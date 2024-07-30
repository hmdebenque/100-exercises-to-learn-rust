use std::net::SocketAddr;
use std::panic;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task::JoinSet;

use outro_08::web_server::server;

#[tokio::test]
async fn test_crud() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    println!("Spawning server");
    tokio::spawn(server(listener));

    let mut jh: JoinSet<()> = JoinSet::new();

    jh.spawn(async move {
        assert_eq!(exchange(addr, "POST", r#"{"title": "Ticket title", "description": "ticket description"}"#).await, r#"{"id":0}"#);
        assert_eq!(exchange(addr, "GET", r#"{"id":0}"#).await, r#"{"id":0,"title":"Ticket title","description":"ticket description","status":"ToDo"}"#);
        assert_eq!(exchange(addr, "PATCH", r#"{"id":0, "status": "done"}"#).await, r#"{"id":0,"title":"Ticket title","description":"ticket description","status":"Done"}"#);
        assert_eq!(exchange(addr, "GET", r#"{"id":0}"#).await, r#"{"id":0,"title":"Ticket title","description":"ticket description","status":"Done"}"#);
        assert_eq!(exchange(addr, "GET", r#"{"id":1}"#).await, r#"No ticket with id 1"#);
    });

    while let Some(outcome) = jh.join_next().await {
        if let Err(err) = outcome {
            println!("Error: {err}");
            if let Ok(reason) = err.try_into_panic() {
                panic::resume_unwind(reason);
            }
        }
    }
}

async fn exchange(addr: SocketAddr, command: &str, data: &str) -> String {
    let req = format!("{command}\n{data}");

    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
    let (mut reader, mut writer) = socket.split();

    // Write request
    writer.write_all(req.as_bytes()).await.unwrap();
    writer.shutdown().await.unwrap();
    println!("CLIENT: Request written: {req}");

    // Read response
    let mut resp_buffer = Vec::new();
    let _ = reader.read_to_end(&mut resp_buffer).await.unwrap();
    let resp_str = String::from_utf8(resp_buffer).unwrap();
    println!("CLIENT: Response is {resp_str}");
    resp_str
}
