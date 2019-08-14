#![feature(proc_macro_hygiene, decl_macro)]

mod random;
mod reverse_turbofish;
mod turbofish;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use rocket::{
    get,
    response::{NamedFile, Redirect},
    routes, uri,
};
use rocket_contrib::templates::Template;

use self::{random::random_type, reverse_turbofish::ReverseTurboFish, turbofish::TurboFish};

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("guts", "");

    Template::render("turbofish", context)
}

#[get("/random")]
fn random() -> Redirect {
    Redirect::to(uri!(turbofish: TurboFish::new(random_type())))
}

#[get("/<turbofish>", rank = 1)]
fn turbofish(turbofish: TurboFish) -> Template {
    let mut context = HashMap::new();
    context.insert("guts", turbofish.gut());

    Template::render("turbofish", context)
}

#[get("/<reverse_turbofish>", rank = 2)]
fn reverse_turbofish(reverse_turbofish: ReverseTurboFish) -> Template {
    let mut context = HashMap::new();
    context.insert("guts", reverse_turbofish.gut());

    Template::render("reverse_turbofish", context)
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
            routes![index, random, turbofish, reverse_turbofish, files],
        )
        .attach(Template::fairing())
        .launch();
}
