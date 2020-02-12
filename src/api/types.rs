#[derive(Serialize, Deserialize, Debug)]
struct Post {
    id: String,
    date_time: String,
    title: String,
    tags: Vec<String>,
    content: String,
}
