use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use crate::{turbofish::TurboFish, HtmlTemplate};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTpl;

pub async fn index() -> impl IntoResponse {
    HtmlTemplate(IndexTpl)
}

pub async fn random() -> impl IntoResponse {
    Redirect::to(format!("/{}", TurboFish::random()).parse().unwrap())
}

pub async fn random_reverse() -> impl IntoResponse {
    Redirect::to(format!("/{}", TurboFish::random_reverse()).parse().unwrap())
}

#[derive(Template)]
#[template(path = "turbofish.html")]
struct TurboFishTpl {
    guts: String,
    guts_link: String,
    reverse: bool,
}

impl TurboFishTpl {
    fn new(turbofish: TurboFish) -> Self {
        Self {
            guts: turbofish.guts.replace("<", "<​"),
            guts_link: utf8_percent_encode(&turbofish.guts, NON_ALPHANUMERIC).to_string(),
            reverse: turbofish.reverse,
        }
    }
}

pub async fn turbofish(Path(turbofish): Path<TurboFish>) -> impl IntoResponse {
    HtmlTemplate(TurboFishTpl::new(turbofish))
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTpl;

pub async fn page_not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, HtmlTemplate(NotFoundTpl))
}
