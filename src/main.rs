#[macro_use]
extern crate rocket;

mod about_me;
mod blog;
mod errors;
mod socials;

use std::collections::HashMap;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;

#[get("/")]
async fn index() -> Option<Template> {
    let context: HashMap<String, String> = HashMap::new();
    Some(Template::render("index", context))
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![index])
        .mount("/", FileServer::from(relative!("static")))
        .mount("/blog", routes![blog::page, blog::index])
        .mount("/links", routes![socials::index])
        .mount("/about-me", routes![about_me::about])
        .register("/", catchers![errors::not_found, errors::internal_death])
        .attach(Template::fairing())
        .launch()
        .await;
}
