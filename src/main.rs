use axum::{
    Router,
    http::StatusCode,
    response::Html,
    routing::get,
};

use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/about", get(about))
        .route("/projects", get(projects))
        .route("/projects/noivern", get(noivern))
        .route("/blog", get(blog))
        .route("/contact", get(contact))
        .nest_service(
            "/downloads/noivern",
            ServeDir::new("releases/noivern"),
        )
        .fallback_service(ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    println!("Server running on http://0.0.0.0:8080");

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn home() -> Result<Html<String>, StatusCode> {
    serve_html("static/index.html").await
}

async fn about() -> Result<Html<String>, StatusCode> {
    serve_html("static/about.html").await
}

async fn projects() -> Result<Html<String>, StatusCode> {
    serve_html("static/projects.html").await
}

async fn noivern() -> Result<Html<String>, StatusCode> {
    serve_html("static/noivern.html").await
}

async fn blog() -> Result<Html<String>, StatusCode> {
    serve_html("static/blog.html").await
}

async fn contact() -> Result<Html<String>, StatusCode> {
    serve_html("static/contact.html").await
}

async fn serve_html(path: &str) -> Result<Html<String>, StatusCode> {
    match tokio::fs::read_to_string(path).await {
        Ok(content) => Ok(Html(content)),

        Err(error) => {
            eprintln!("Could not read {path}: {error}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}