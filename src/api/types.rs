#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    id: String,
    date_time: String,
    title: String,
    tags: Vec<String>,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    username: String,
    password: String,
}
