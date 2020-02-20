use crate::schema::posts;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[database("postgres_db")]
pub struct DBConn(PgConnection);

#[derive(Insertable, Serialize, Debug)]
#[table_name = "posts"]
pub struct NewPost {
    title: String,
    body: String,
    published: bool,
    date_published: NaiveDateTime,
    date_updated: Option<NaiveDateTime>,
    tags: String,
}

#[derive(Serialize, Debug)]
pub struct Post {
    title: String,
    body: String,
    published: bool,
    date_published: NaiveDateTime,
    date_updated: Option<NaiveDateTime>,
    tags: String,
}

#[derive(Queryable, Serialize, Debug)]
pub struct Posts {
    entries: Vec<Post>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    username: String,
    password: String,
}
