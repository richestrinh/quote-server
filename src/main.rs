use askama::Template;
use axum::{
    response::Html,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer; // For logging and tracing
use tokio::net::TcpListener;

#[derive(Template)]
#[template(path = "quote.html")]
struct QuoteTemplate {
    quote: &'static str,
    author: &'static str,
}

async fn show_quote() -> Html<String> {
    let template = QuoteTemplate {
        quote: "The only limit to our realization of tomorrow is our doubts of today.",
        author: "Franklin D. Roosevelt",
    };
    
    match template.render() {
        Ok(rendered) => Html(rendered),
        Err(err) => {
            eprintln!("Template rendering failed: {}", err);
            Html("<h1>Error rendering page</h1>".to_string())
        }
    }
}

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    // Create the application router
    let app = Router::new()
        .route("/", get(show_quote))
        .layer(TraceLayer::new_for_http()); // Layer for tracing/logging

    // Set the address to listen on
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000)); // Bind to all network interfaces
    println!("Listening on {}", addr);

    // Use a TcpListener and pass it to axum::serve directly
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = serve().await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}
