use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Project {
    pub id: usize,
    pub name: String,
    pub url: String,
    pub watchers: usize,
    pub forks: usize,
    pub stars: usize,
}

#[derive(Serialize, Deserialize)]
pub struct NewProject {
    pub name: String,
    pub url: String,
    pub watchers: usize,
    pub forks: usize,
    pub stars: usize,
}

pub struct Issue {
    author: String,
    url: String,
    title: String,
}
