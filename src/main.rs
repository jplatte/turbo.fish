#![feature(non_ascii_idents, plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

mod turbofish;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket_contrib::Template;

use turbofish::TurboFish;

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("guts", "");

    Template::render("turbofish", context)
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
        .mount("/", routes![index, turbofish, files])
        .attach(Template::fairing())
        .launch();
}
