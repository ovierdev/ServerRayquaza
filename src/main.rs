use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!("Server run in http://127.0.0.1:8080");

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}
async fn process(mut socket: TcpStream) {
    let mut buffer = [0; 1024];

    socket.read(&mut buffer).await.expect("Error read request");

    let request = String::from_utf8_lossy(&buffer);

    let path = if request.starts_with("GET /style.css") {
        "static/style.css"
    } else {
        "static/index.html"
    };

    let content_type = if path.ends_with(".css") {
        "text/css; charset=UTF-8"
    } else {
        "text/html; charset=UTF-8"
    };

    let content = tokio::fs::read_to_string(path)
        .await
        .unwrap_or_else(|_| "<h1>404 - File not found</h1>".to_string());

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        content_type,
        content.len(),
        content
    );

    socket
        .write_all(response.as_bytes())
        .await
        .expect("Error writing response");
}
