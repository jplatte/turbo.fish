use std::{convert::Infallible, net::SocketAddr};

use axum::{
    body::{Bytes, Full},
    handler::{get, Handler},
    http::{Response, StatusCode},
    response::{Html, IntoResponse},
    service, Router,
};
use tower_http::services::ServeDir;

mod random;
mod routes;
mod turbofish;

#[tokio::main]
async fn main() -> Result<(), axum::BoxError> {
    let app = Router::new()
        .route("/", get(routes::index))
        .route("/random", get(routes::random))
        .route("/random_reverse", get(routes::random_reverse))
        .route("/:turbofish", get(routes::turbofish))
        .or(service::get(ServeDir::new("static")).handle_error(|error: std::io::Error| {
            Ok::<_, Infallible>((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            ))
        }))
        .or(routes::page_not_found.into_service());

    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 8001)))
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// Taken from https://github.com/tokio-rs/axum/blob/02e61dfdd6f05cd87f2edc9475b47974a50ce9f7/examples/templates/src/main.rs
struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: askama::Template,
{
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::from(format!("Failed to render template. Error: {}", err)))
                .unwrap(),
        }
    }
}
