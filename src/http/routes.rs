use crate::api::types::{Post, User, DBConn};

#[get("/", format = "json")]
pub fn get_posts(conn: DBConn) {
    
}

#[get("/<id>", format = "json")]
pub fn get_post(id: usize) {}

#[post("/new", format = "application/json", data = "<post>")]
pub fn new_post(post: String) {}

#[get("/<id>", format = "json")]
pub fn get_user(id: usize) {}
