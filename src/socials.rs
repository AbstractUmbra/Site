use comrak::{markdown_to_html, ComrakOptions};
use rocket::{fs::relative, serde::Serialize};
use rocket_dyn_templates::Template;
use std::fs;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct SocialsContext {
    text: String,
}

fn get_socials() -> String {
    let contents = fs::read_to_string(relative!("misc/socials.md")).unwrap();
    let mut options = ComrakOptions::default();
    options.extension.table = true;

    markdown_to_html(&contents, &options)
}

#[get("/")]
pub async fn index() -> Option<Template> {
    let text = get_socials();
    let context = SocialsContext { text };
    Some(Template::render("socials", context))
}
