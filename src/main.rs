use rocket_dyn_templates::{Template, context};
use rocket::fs::FileServer;

use std::fs;

#[macro_use] extern crate rocket;

fn get_gallery_paths(location: &str) -> Vec<std::string::String> {
    let dir_path = format!("./static/img/Gallery/{}", location);
    return fs::read_dir(dir_path)
    .unwrap()
    .filter_map(|e| e.ok())
    .map(|e| e.path().file_stem().expect("jpegs").to_string_lossy().into_owned())
    .collect::<Vec<_>>();
}

#[get("/")]
fn index() -> Template {
    Template::render("home", context! {})
}

#[get("/projects")]
fn projects() -> Template {
    Template::render("projects", context! {})
}

#[get("/gallery")]
fn gallery() -> Template {
    let paths_northumberland = get_gallery_paths("Northumberland");
    let paths_wales = get_gallery_paths("Wales");
    Template::render("gallery", context! {paths_wales, paths_northumberland})
}

#[get("/cv")]
fn cv() -> Template {
    Template::render("cv", context! {})
}

#[get("/contact-me")]
fn contact() -> Template {
    Template::render("contact", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, projects, gallery, cv, contact])
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
}