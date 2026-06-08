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
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("/");
    let route = first_line.split_whitespace().nth(1).unwrap_or("/");

    println!("[Info] Path request: {}", route);
    println!("[Info] Method: {}", method);

    if method == "POST" && route == "/contact" {
        let body = request.split("\r\n\r\n").nth(1).unwrap_or("");
        println!("[Info] Body raw: {}", body);

        let html = render_thank_you_page(body);

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {}\r\n\r\n{}",
            html.len(),
            html
        );
        let _ = socket
            .write_all(response.as_bytes())
            .await;

        return;
    }

    let file_path = if route.starts_with("/images/") {
        format!("static{}", route)
    } else {
        match route {
            "/" => "static/index.html",
            "/about" => "static/about.html",
            "/projects" => "static/projects.html",
            "/contact" => "static/contact.html",
            "/blog" => "static/blog.html",
            "/style.css" => "static/style.css",
            _ => "static/404.html",
        }
        .to_string()
    };

    let status_line = if file_path == "static/404.html" {
        "HTTP/1.1 404 NOT FOUND"
    } else {
        "HTTP/1.1 200 OK"
    };

    let content_type = get_content_type(&file_path);

    let is_binary = content_type.starts_with("image/");

    let content: Vec<u8> = if is_binary {
        tokio::fs::read(&file_path)
            .await
            .unwrap_or_else(|_| b"<h1>404 - File not found</h1>".to_vec())
    } else {
        tokio::fs::read_to_string(&file_path)
            .await
            .unwrap_or_else(|_| "<h1>404 - File not found</h1>".to_string())
            .into_bytes()
    };

    let response = format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
        status_line,
        content_type,
        content.len(),
    );

    let _ = socket
        .write_all(response.as_bytes())
        .await;

    let _ = socket
        .write_all(&content)
        .await;
}

fn get_content_type(file_path: &str) -> &str {
    if file_path.ends_with(".html") {
        "text/html; charset=UTF-8"
    } else if file_path.ends_with(".css") {
        "text/css; charset=UTF-8"
    } else if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") {
        "image/jpeg"
    } else if file_path.ends_with(".png") {
        "image/png"
    } else if file_path.ends_with(".svg") {
        "image/svg+xml"
    } else {
        "application/octet-stream"
    }
}

fn render_thank_you_page(body: &str) -> String {
    let mut name = "";
    let mut message = "";

    for pair in body.split('&') {
        let mut parts = pair.split('=');

        let key = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("");

        if key == "name" {
            name = value;
        }
        if key == "message" {
            message = value;
        }
    }

    let name = decode_form_value(name);
    let message = decode_form_value(message);

    format!(
        r#"
<!doctype html>
<html lang="es">
<head>
    <meta charset="UTF-8"/>
    <title> Message sent | Ovier </title>
    <link rel="stylesheet" href="/style.css" />
</head>
<body>
    <nav>
        <a href="/">Home</a>
        <a href="/about">About Me </a>
        <a href="/projects"> Projects </a>
        <a href="/contact">Contact </a>
        <a href="/blog>Blog </a>
    </nav>

    <main>
        <section class="card">
            <h1>Thanks, <span class="accent">{}</span></h1>
            <p> Your message was recived:</p>
            <p><strong>{}</strong></p>
            <a href="/contact">Send another message</a>
        </section>
    </main>
</body>
</html>
"#,
        name, message
    )
}

fn decode_form_value(value: &str) -> String {
    value.replace('+', " ")
}
