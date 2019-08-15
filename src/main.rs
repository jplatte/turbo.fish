#![feature(proc_macro_hygiene, decl_macro)]

mod random;
mod reverse_turbofish;
mod turbofish;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use rocket::{
    get,
    response::{NamedFile, Redirect},
    routes, uri,
};
use rocket_contrib::templates::Template;

use self::{random::random_type, reverse_turbofish::ReverseTurboFish, turbofish::TurboFish};

fn tpl_context(guts: &str) -> HashMap<&'static str, String> {
    let mut context = HashMap::new();
    context.insert("guts", guts.replace("<", "<​"));
    context.insert(
        "guts_link",
        utf8_percent_encode(guts, NON_ALPHANUMERIC).to_string(),
    );

    context
}

#[get("/")]
fn index() -> Template {
    Template::render("turbofish", tpl_context(""))
}

#[get("/random")]
fn random() -> Redirect {
    Redirect::to(uri!(turbofish: TurboFish::new(random_type())))
}

#[get("/random_reverse")]
fn random_reverse() -> Redirect {
    Redirect::to(uri!(
        reverse_turbofish: ReverseTurboFish::new(random_type())
    ))
}

#[get("/<turbofish>", rank = 1)]
fn turbofish(turbofish: TurboFish) -> Template {
    Template::render("turbofish", tpl_context(&turbofish.gut()))
}

#[get("/<reverse_turbofish>", rank = 2)]
fn reverse_turbofish(reverse_turbofish: ReverseTurboFish) -> Template {
    Template::render("reverse_turbofish", tpl_context(&reverse_turbofish.gut()))
}

// From https://github.com/SergioBenitez/Rocket/blob/master/examples/static_files/src/main.rs
#[get("/<file..>", rank = 100)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    // TODO: Custom 404
    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                random,
                random_reverse,
                turbofish,
                reverse_turbofish,
                files
            ],
        )
        .attach(Template::fairing())
        .launch();
}
