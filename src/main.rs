use rocket_dyn_templates::{Template, context};
use rocket::fs::FileServer;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
    Template::render("home", context! {})
}

#[get("/projects")]
fn projects() -> Template {
    Template::render("projects", context! {})
}

#[get("/cv")]
fn cv() -> Template {
    Template::render("cv", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, projects, cv])
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
}