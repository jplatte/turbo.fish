use askama::Template;
use axum::{
    extract::{rejection::PathParamsRejection, Path},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use percent_encoding::utf8_percent_encode;

use crate::{turbofish::TurboFish, HtmlTemplate, FRAGMENT};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTpl;

pub async fn index() -> impl IntoResponse {
    HtmlTemplate(IndexTpl)
}

pub async fn random() -> impl IntoResponse {
    Redirect::to(format!("/{}", TurboFish::random().to_uri_segment()).parse().unwrap())
}

pub async fn random_reverse() -> impl IntoResponse {
    Redirect::to(format!("/{}", TurboFish::random_reverse().to_uri_segment()).parse().unwrap())
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
            guts: turbofish.guts.replace("<", "<\u{200B}"),
            guts_link: utf8_percent_encode(&turbofish.guts, FRAGMENT).to_string(),
            reverse: turbofish.reverse,
        }
    }
}

pub async fn turbofish(path: Result<Path<TurboFish>, PathParamsRejection>) -> impl IntoResponse {
    path.map(|Path(turbofish)| HtmlTemplate(TurboFishTpl::new(turbofish)))
        .map_err(|_| (StatusCode::NOT_FOUND, HtmlTemplate(NotFoundTpl)))
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTpl;

pub async fn page_not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, HtmlTemplate(NotFoundTpl))
}
