use rocket::serde::Serialize;
use rocket::Request;
use rocket_dyn_templates::Template;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct ErrorContext {
    title: String,
    message: String,
}

#[catch(404)]
pub fn not_found(_: &Request<'_>) -> Template {
    let context = ErrorContext {
        title: "Fuck.".to_string(),
        message: "Seems you're looking in places you shouldn't be.".to_string(),
    };
    Template::render("errors/generic", context)
}

#[catch(500)]
pub fn internal_death(_: &Request<'_>) -> Template {
    let context = ErrorContext {
        title: "Fuck".to_string(),
        message: "It seems like something inside broke.".to_string(),
    };
    Template::render("errors/generic", context)
}
