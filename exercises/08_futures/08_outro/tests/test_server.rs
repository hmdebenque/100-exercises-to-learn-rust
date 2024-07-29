use tokio::net::TcpListener;
use outro_08::web_server::server;

#[tokio::test]
async fn test_crud() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        server(port);
    });
}
