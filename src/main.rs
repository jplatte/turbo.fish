use std::{net::Ipv4Addr, process::ExitCode, sync::Arc};

use axum::{routing::get, Router};
use minijinja::Environment;
use percent_encoding::{AsciiSet, CONTROLS};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod random;
mod routes;
mod turbofish;

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

const TPL_404: &str = include_str!("../templates/404.html");
const TPL_ABOUT: &str = include_str!("../templates/about.html");
const TPL_INDEX: &str = include_str!("../templates/index.html");
const TPL_SKEL: &str = include_str!("../templates/skel.html");
const TPL_TURBOFISH: &str = include_str!("../templates/turbofish.html");

fn main() -> ExitCode {
    match tokio::runtime::Runtime::new()
        .expect("Failed to build the tokio Runtime")
        .block_on(async_main())
    {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            // Don't return `Result` from `main` as that would print the
            // `Debug` formatting of the error, `Display` is nicer.
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

async fn async_main() -> Result<(), axum::BoxError> {
    let mut minijinja_env = Environment::new();
    minijinja_env.add_template("404", TPL_404)?;
    minijinja_env.add_template("about", TPL_ABOUT)?;
    minijinja_env.add_template("index", TPL_INDEX)?;
    minijinja_env.add_template("skel", TPL_SKEL)?;
    minijinja_env.add_template("turbofish", TPL_TURBOFISH)?;

    let app = Router::new()
        .route("/", get(routes::index))
        .route("/about", get(routes::about))
        .route("/random", get(routes::random))
        .route("/random_reverse", get(routes::random_reverse))
        .route("/:turbofish", get(routes::turbofish))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(routes::page_not_found)
        .with_state(Arc::new(minijinja_env));

    println!("Starting server at http://localhost:8001/");
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 8001)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
