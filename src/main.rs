mod random;
mod routes;
mod turbofish;

use rocket::{catchers, routes};
use rocket_dyn_templates::Template;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount(
            "/",
            routes![
                routes::index,
                routes::random,
                routes::random_reverse,
                routes::turbofish,
                routes::files,
            ],
        )
        .register("/", catchers![routes::page_not_found])
        .attach(Template::fairing())
        .launch()
        .await
}
