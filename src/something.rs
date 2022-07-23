use comrak::{markdown_to_html, ComrakOptions};
use rocket::{fs::relative, serde::Serialize};
use rocket_dyn_templates::Template;
use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
    time::SystemTime,
};

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Context {
    files: Vec<(PathBuf, String, u64)>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct TextContext {
    title: String,
    text: String,
    song: Option<String>,
}

pub fn get_filename_to_title(path: PathBuf) -> String {
    path.file_stem()
        .and_then(|filename| filename.to_str())
        .map(|filename| {
            filename
                .split('_')
                .filter_map(|word| {
                    let mut characters = word.chars();
                    let first = characters.next()?;
                    let result = first.to_uppercase().chain(characters).collect::<String>();
                    Some(result)
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_else(|| "<no title>".to_owned())
}

fn get_pages() -> Vec<(PathBuf, String, u64)> {
    let mut paths: Vec<_> = fs::read_dir("pages/")
        .unwrap()
        .filter_map(|result| {
            let entry: DirEntry = result.ok()?;
            let created: SystemTime = entry
                .metadata()
                .and_then(|metadata| metadata.created())
                .ok()?;

            let path = entry.path();
            let name = get_filename_to_title(entry.path());

            let file_name = !entry.path().file_name()?.to_string_lossy().starts_with('.');
            file_name.then(|| {
                (
                    path,
                    name,
                    created
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                )
            })
        })
        .collect();

    paths.sort_by(|(_, _, a), (_, _, b)| a.cmp(b));

    paths
}

#[get("/")]
pub async fn index() -> Option<Template> {
    let files = get_pages();
    let context = Context { files };
    Some(Template::render("something", context))
}

#[get("/pages/<file..>")]
pub async fn page(file: PathBuf) -> Option<Template> {
    let path = Path::new(relative!("pages/")).join(&file);
    let markdown = rocket::tokio::fs::read_to_string(&path)
        .await
        .expect("Unable to read file.");

    let mut options = ComrakOptions::default();
    options.extension.strikethrough = true;
    let text = markdown_to_html(&markdown, &options);
    let title = get_filename_to_title(path);
    let song_file = Path::new(relative!("songs/"))
        .join(file)
        .with_extension("song");
    let song = rocket::tokio::fs::read_to_string(song_file).await.ok();

    let context = TextContext { title, text, song };
    Some(Template::render("pages", context))
}
