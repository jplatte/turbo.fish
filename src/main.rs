use std::{
    convert::Infallible,
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::{error_handling::HandleError, http::StatusCode, routing::get, Router};
use minijinja::Environment;
use percent_encoding::{AsciiSet, CONTROLS};
use tokio::signal;
use tower_http::services::ServeDir;

mod random;
mod routes;
mod turbofish;

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

const TPL_404: &str = include_str!("../templates/404.html");
const TPL_INDEX: &str = include_str!("../templates/index.html");
const TPL_SKEL: &str = include_str!("../templates/skel.html");
const TPL_TURBOFISH: &str = include_str!("../templates/turbofish.html");

#[tokio::main]
async fn main() -> Result<(), axum::BoxError> {
    let mut minijinja_env = Environment::new();
    minijinja_env.add_template("404", TPL_404)?;
    minijinja_env.add_template("index", TPL_INDEX)?;
    minijinja_env.add_template("skel", TPL_SKEL)?;
    minijinja_env.add_template("turbofish", TPL_TURBOFISH)?;

    let app = Router::new()
        .route("/", get(routes::index))
        .route("/random", get(routes::random))
        .route("/random_reverse", get(routes::random_reverse))
        .route("/:turbofish", get(routes::turbofish))
        .nest_service(
            "/static",
            HandleError::new(ServeDir::new("static"), |error: std::io::Error| async move {
                Ok::<_, Infallible>((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                ))
            }),
        )
        .fallback(routes::page_not_found)
        .with_state(Arc::new(minijinja_env));

    println!("Starting server at http://localhost:8001/");
    axum::Server::bind(&SocketAddr::from((Ipv4Addr::LOCALHOST, 8001)))
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
