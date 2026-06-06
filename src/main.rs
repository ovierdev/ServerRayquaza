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

    socket.read(&mut buffer).await.unwrap();

    let html = r#"
        <!DOCTYPE html>
        <html lang ="es">
        <head>
            <meta charset="UTF-8">
            <title> Ovier | Presentation </title>
            <style>
                body{
                    background: #111827;
                    color: #F9FAFB;
                    font-family: Arial, sans-serif;
                    margin: 0;
                    padding: 0;
                }
                main {
                    min-height: 100vh;
                    display: flex;
                    justfify-content: center;
                    align-items: center;
                }
                .card {
                    max-width: 700px;
                    padding: 40px;
                    border-radius: 20px;
                    background: #1F2937;
                    box-shadow: 0 20px 60px rgba(0,0,0,0.4);
                }
                h1 {
                    font-size: 48px;
                    margin-bottom: 10px;
                }
                .accent{
                    color: #38BDF8;
                }
                p {
                    font-size: 20px;
                    line-height: 1.6;
                    color: #D1D5DB;
                }
                .tags {
                    margin-top: 25px;
                }
                .tags {
                    display: inline-block;
                    margin: 5px;
                    padding: 8px 14px;
                    background: #374151;
                    border-radius: 999px;
                    color: #E5E7EB;
                }
            </style>
        </head>
        <body>
            <main>
                <section class="card">
                    <h1> Hola, I'm <span class="accent">Ovier</span></h1>
                    <p>
                        I work in Semiconductors area and I'm learning Rust,
                        Linux, backend and systems. This page have server Rust Tokio.
                    </p>

                    <div class="tags">
                        <span class="tag">Rust</span>
                        <span class="tag">Tokio</sapn>
                        <span class="tag">Linux</span>
                        <span class="tag>Semiconductors</span>
                        <span class="tag">Backend</span>
                    </div>
                </section>
            </main>
        </body>
        </html>
        "#;
    let response = format!(
        "HTTP/1.1 200 ok\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Lenght: {}\r\n\r\n{}",
        html.len(),
        html
    );

    socket.write_all(response.as_bytes()).await.unwrap();
}
