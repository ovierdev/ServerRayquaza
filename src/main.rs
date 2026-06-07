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
    let first_line = request.lines().next().unwrap_or("");

    println!("[Info] Request: {}", first_line);

    let route = first_line.split_whitespace().nth(1).unwrap_or("/");

    println!("Path request: {}", route);

    let file_path = match route {
        "/" => "static/index.html",
        "/about" => "static/about.html",
        "/projects" => "static/projects.html",
        "/contact" => "static/contact.html",
        "/blog" => "static/blog.html",
        "/style.css" => "static/style.css",
        _ => "static/404.html",
    };

    let status_line = if file_path == "static/404.html" {
        "HTTP/1.1 404 NOT FOUND"
    } else {
        "HTTP/1.1 200 OK"
    };

    let content_type = if file_path.ends_with(".css") {
        "text/css; charset=UTF-8"
    } else {
        "text/html; charset=UTF-8"
    };

    let content = tokio::fs::read_to_string(file_path)
        .await
        .unwrap_or_else(|_| "<h1>404 - File not found</h1>".to_string());

    let response = format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content_type,
        content.len(),
        content
    );

    socket
        .write_all(response.as_bytes())
        .await
        .expect("Error writing response");
}
