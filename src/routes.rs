use std::sync::Arc;

use axum::{
    extract::{Path, State, rejection::PathRejection},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use minijinja::{Environment, context};
use percent_encoding::utf8_percent_encode;
use serde::Serialize;

use crate::{FRAGMENT, turbofish::TurboFish};

pub async fn index(env: State<Arc<Environment<'static>>>) -> impl IntoResponse {
    render_html_template(&env, "index", ())
}

pub async fn about(env: State<Arc<Environment<'static>>>) -> impl IntoResponse {
    render_html_template(&env, "about", ())
}

pub async fn random() -> impl IntoResponse {
    Redirect::to(&format!("/{}", TurboFish::random().to_uri_segment()))
}

pub async fn random_reverse() -> impl IntoResponse {
    Redirect::to(&format!("/{}", TurboFish::random_reverse().to_uri_segment()))
}

pub async fn turbofish(
    env: State<Arc<Environment<'static>>>,
    path: Result<Path<TurboFish>, PathRejection>,
) -> impl IntoResponse {
    match path {
        Ok(Path(turbofish)) => Ok(render_html_template(
            &env,
            "turbofish",
            context! {
                guts => turbofish.guts.replace('<', "<\u{200B}"),
                guts_link => utf8_percent_encode(&turbofish.guts, FRAGMENT).to_string(),
                reverse => turbofish.reverse,
            },
        )),
        Err(_) => Err(page_not_found(env).await),
    }
}

pub async fn page_not_found(env: State<Arc<Environment<'static>>>) -> impl IntoResponse {
    render_html_template(&env, "404", ()).map(|ok| (StatusCode::NOT_FOUND, ok))
}

fn render_html_template<S>(
    env: &Environment<'_>,
    name: &str,
    ctx: S,
) -> Result<Html<String>, impl IntoResponse + use<S>>
where
    S: Serialize,
{
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
}
