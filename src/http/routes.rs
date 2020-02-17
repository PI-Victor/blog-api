use crate::api::types::{Post, User};

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

#[get("/posts", format = "json")]
pub fn get_posts() -> JsonValue {
    json!({})
}

#[get("/post/<id>", format = "json")]
pub fn get_post(id: usize) -> JsonValue {
    json!({})
}

#[post("/post/new", format = "application/json", data = "<post>")]
pub fn new_post(post: String) {}

#[get("/user/<id>", format = "json")]
pub fn get_user(id: usize) -> JsonValue {
    json!({})
}
