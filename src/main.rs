#![feature(plugin)]
#![plugin(rocket_codegen)]

mod random;
mod turbofish;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use rocket::{
    http::uri::URI,
    response::{NamedFile, Redirect},
};
use rocket_contrib::Template;

use self::{random::random_type, turbofish::TurboFish};

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("guts", "");

    Template::render("turbofish", context)
}

#[get("/random")]
fn random() -> Redirect {
    // Safari doesn't seem to like redirect URLs starting with ::
    // the leading / is here to fix that.
    Redirect::to(&format!(
        "/{}",
        URI::percent_encode(&format!("::<{}>", random_type()))
    ))
}

#[get("/<turbofish>")]
fn turbofish(turbofish: TurboFish) -> Template {
    let mut context = HashMap::new();
    context.insert("guts", turbofish.gut());

    Template::render("turbofish", context)
}

// From https://github.com/SergioBenitez/Rocket/blob/master/examples/static_files/src/main.rs
#[get("/<file..>", rank = 100)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    // TODO: Custom 404
    rocket::ignite()
        .mount("/", routes![index, random, turbofish, files])
        .attach(Template::fairing())
        .launch();
}
