use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use rocket::{
    catch,
    fs::NamedFile,
    get,
    response::{status::NotFound, Redirect},
    uri,
};
use rocket_dyn_templates::Template;
use serde_json::json;

use crate::{random::random_type, reverse_turbofish::ReverseTurboFish, turbofish::TurboFish};

fn tpl_context(guts: &str, reverse: bool) -> serde_json::Value {
    json!({
        "guts": guts.replace("<", "<​"),
        "guts_link": utf8_percent_encode(guts, NON_ALPHANUMERIC).to_string(),
        "reverse": reverse,
    })
}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", HashMap::<String, String>::new())
}

#[get("/random")]
pub fn random() -> Redirect {
    Redirect::to(uri!(turbofish(TurboFish::new(random_type()))))
}

#[get("/random_reverse")]
pub fn random_reverse() -> Redirect {
    Redirect::to(uri!(reverse_turbofish(ReverseTurboFish::new(random_type()))))
}

#[get("/<turbofish>", rank = 1)]
pub fn turbofish(turbofish: TurboFish) -> Template {
    Template::render("turbofish", tpl_context(&turbofish.gut(), true))
}

#[get("/<reverse_turbofish>", rank = 2)]
pub fn reverse_turbofish(reverse_turbofish: ReverseTurboFish) -> Template {
    Template::render("turbofish", tpl_context(&reverse_turbofish.gut(), false))
}

// From https://github.com/SergioBenitez/Rocket/blob/master/examples/static_files/src/main.rs
#[get("/<file..>", rank = 10)]
pub async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[catch(404)]
pub fn page_not_found() -> NotFound<Template> {
    NotFound(Template::render("404", HashMap::<String, String>::new()))
}
