use comrak::{markdown_to_html, ComrakOptions};
use rocket::{fs::relative, serde::Serialize, tokio::fs::read_to_string};
use rocket_dyn_templates::Template;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct AboutContext {
    text: String,
}

async fn get_about() -> String {
    let content = read_to_string(relative!("misc/about.md")).await.unwrap();
    let mut options = ComrakOptions::default();
    options.extension.table = true;

    markdown_to_html(&content, &options)
}

#[get("/")]
pub async fn about() -> Option<Template> {
    let socials = get_about().await;
    let context = AboutContext { text: socials };
    Some(Template::render("about", context))
}
