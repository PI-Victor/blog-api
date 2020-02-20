use crate::api::types::{DBConn, NewPost, NewUser};
use rocket_contrib::json::Json;

#[get("/", format = "json")]
pub fn get_posts(conn: DBConn) {}

#[get("/<id>", format = "json")]
pub fn get_post(id: usize) {}

#[post("/new", format = "application/json", data = "<post>")]
pub fn new_post(conn: DBConn, post: Json<NewPost>) {
    info!("THIS IS MY POST: {:?}", post)
}

#[get("/<id>", format = "json")]
pub fn get_user(id: usize) {}
