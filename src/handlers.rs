use rocket::{get, post};
use rocket_contrib::json::Json;

use crate::types::{Project,NewProject};

#[get("/")]
pub fn root() -> String {
    "It works!".to_string()
}

#[get("/projects")]
pub fn get_projects() -> Json<Vec<Project>> {
    Json(vec![Project {
        id: 1024,
        url: "https://github.com/foo/bar".to_string(),
        name: "Bar Project".to_string(),
        stars: 0,
        watchers: 0,
        forks: 0,
    }])
}

#[post("/projects", data="<new_project>")]
pub fn create_project(new_project: Json<NewProject>) -> Json<Project> {
    Json(Project {
        id: 2048,
        url: new_project.url.to_string(),
        name: new_project.name.to_string(),
        stars: new_project.stars,
        watchers: new_project.watchers,
        forks: new_project.forks,
    })
}
