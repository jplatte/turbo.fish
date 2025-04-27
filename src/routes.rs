use axum::{
    extract::{Path, rejection::PathRejection},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use minijinja::context;
use percent_encoding::utf8_percent_encode;
use serde::Serialize;

use crate::{FRAGMENT, MINIJINJA_ENV, turbofish::TurboFish};

pub async fn index() -> impl IntoResponse {
    render_html_template("index", ())
}

pub async fn about() -> impl IntoResponse {
    render_html_template("about", ())
}

pub async fn random() -> impl IntoResponse {
    Redirect::to(&format!("/{}", TurboFish::random().to_uri_segment()))
}

pub async fn random_reverse() -> impl IntoResponse {
    Redirect::to(&format!("/{}", TurboFish::random_reverse().to_uri_segment()))
}

pub async fn turbofish(path: Result<Path<TurboFish>, PathRejection>) -> impl IntoResponse {
    match path {
        Ok(Path(turbofish)) => Ok(render_html_template(
            "turbofish",
            context! {
                guts => turbofish.guts.replace('<', "<\u{200B}"),
                guts_link => utf8_percent_encode(&turbofish.guts, FRAGMENT).to_string(),
                reverse => turbofish.reverse,
            },
        )),
        Err(_) => Err(page_not_found().await),
    }
}

pub async fn page_not_found() -> impl IntoResponse {
    render_html_template("404", ()).map(|ok| (StatusCode::NOT_FOUND, ok))
}

fn render_html_template<S>(name: &str, ctx: S) -> Result<Html<String>, impl IntoResponse + use<S>>
where
    S: Serialize,
{
    MINIJINJA_ENV.with(|env| {
        let Ok(template) = env.get_template(name) else {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Couldn't find MiniJinja template".to_owned(),
            ));
        };

        let rendered = template.render(ctx).map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to render MiniJinja template: {e}"))
        })?;

        Ok(Html(rendered))
    })
}
