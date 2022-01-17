use std::{convert::Infallible, net::SocketAddr};

use axum::{
    handler::Handler,
    http::StatusCode,
    routing::{get, get_service},
    Router,
};
use percent_encoding::{AsciiSet, CONTROLS};
use tokio::signal;
use tower_http::services::ServeDir;

mod random;
mod routes;
mod turbofish;

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

#[tokio::main]
async fn main() -> Result<(), axum::BoxError> {
    let app = Router::new()
        .route("/", get(routes::index))
        .route("/random", get(routes::random))
        .route("/random_reverse", get(routes::random_reverse))
        .route("/:turbofish", get(routes::turbofish))
        .nest(
            "/static",
            get_service(ServeDir::new("static")).handle_error(|error: std::io::Error| async move {
                Ok::<_, Infallible>((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                ))
            }),
        )
        .fallback(routes::page_not_found.into_service());

    println!("Starting server at http://localhost:8001/");
    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 8001)))
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
