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

#[get("/robots.txt")]
async fn robots() -> Template {
    const DEFAULT_AI_LIST: String = String::new();

    // Requests a list of AI scrapers from the "ai.robots.txt" project
    // This list is added to the default robots template, to keep scraper-blocking up-to-date
    let ai_list_fetch = reqwest::get("https://raw.githubusercontent.com/ai-robots-txt/ai.robots.txt/refs/heads/main/robots.txt")
        .await;
    let ai_bots = match ai_list_fetch {
        Ok(response) => {
            let mut response_text = response.text().await.unwrap_or(DEFAULT_AI_LIST);
            if response_text == "404: Not Found" {
                error!("Failed to fetch AI Robots list: 404, resource may have moved or been removed");
                response_text = DEFAULT_AI_LIST;
            }
            response_text
        },
        Err(e) => {
            error!("Failed to fetch AI Robots list: {}", e);
            DEFAULT_AI_LIST
        }
    };

    Template::render("robots", context! {ai_bots})
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


// Project Pages
#[get("/projects/pathfinding")]
fn projects_pathfinding() -> Template {
    Template::render("projects/pathfinding", context! {})
}
#[get("/projects/allendale-holidays-website")]
fn projects_allendale_holidays() -> Template {
    Template::render("projects/allendale-holidays-website", context! {})
}
#[get("/projects/bae-systems-ctf")]
fn projects_bae_systems_ctf() -> Template {
    Template::render("projects/bae-systems-ctf", context! {})
}
#[get("/projects/battlefield-labyrinth")]
fn projects_battlefield_labyrinth() -> Template {
    Template::render("projects/battlefield-labyrinth", context! {})
}
#[get("/projects/festival-of-code")]
fn projects_festival_of_code() -> Template {
    Template::render("projects/festival-of-code", context! {})
}
#[get("/projects/google-hashcode")]
fn projects_google_hashcode() -> Template {
    Template::render("projects/google-hashcode", context! {})
}
#[get("/projects/lora-chat")]
fn projects_lora_chat() -> Template {
    Template::render("projects/lora-chat", context! {})
}
#[get("/projects/recipe-viewer")]
fn projects_recipe_viewer() -> Template {
    Template::render("projects/recipe-viewer", context! {})
}
#[get("/projects/personal-website")]
fn projects_personal_website() -> Template {
    Template::render("projects/personal-website", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            index, robots, projects, gallery, cv, contact, 
            projects_pathfinding, projects_allendale_holidays, projects_bae_systems_ctf, projects_battlefield_labyrinth, projects_festival_of_code, 
            projects_google_hashcode, projects_lora_chat, projects_recipe_viewer, projects_personal_website
        ])
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
}