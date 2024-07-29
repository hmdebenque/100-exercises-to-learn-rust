use tokio::main;
use tokio::net::TcpListener;

#[main]
pub async fn server(port: u16) {
    let listener = TcpListener::bind( format!("127.0.0.1:{port}")).await.unwrap();
    let mut stopped = false;
    while !stopped {
        let (stream, socket) = listener.accept().await.unwrap();
        
        
        
        
        stopped = true;
    }
}


