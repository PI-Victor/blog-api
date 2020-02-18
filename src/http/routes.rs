use crate::api::types::{Post, User};

#[get("/", format = "json")]
pub fn get_posts() {}

#[get("/post/<id>", format = "json")]
pub fn get_post(id: usize) {}

#[post("/post/new", format = "application/json", data = "<post>")]
pub fn new_post(post: String) {}

#[get("/user/<id>", format = "json")]
pub fn get_user(id: usize) {}
