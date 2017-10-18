#![feature(non_ascii_idents, plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

mod turbo_fish;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket_contrib::Template;

use turbo_fish::TurboFish;

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("guts", "");

    Template::render("turbo_fish", context)
}

#[get("/<turbo_fish>")]
fn turbo_fish(turbo_fish: TurboFish) -> Template {
    let mut context = HashMap::new();
    context.insert("guts", turbo_fish.gut());

    Template::render("turbo_fish", context)
}

// From https://github.com/SergioBenitez/Rocket/blob/master/examples/static_files/src/main.rs
#[get("/<file..>", rank = 100)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    // TODO: Custom 404
    rocket::ignite()
        .mount("/", routes![index, turbo_fish, files])
        .attach(Template::fairing())
        .launch();
}
