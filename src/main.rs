#![feature(proc_macro_hygiene, decl_macro)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

use rocket::fairing::AdHoc;
use rocket::Rocket;

mod api;
mod http;
mod schema;

use http::routes as endpoints;
use http::{catchers, guards};

use api::types::DBConn;

embed_migrations!();

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(DBConn::fairing())
        .attach(AdHoc::on_attach("Database migrations", run_migration))
        .mount("/post", routes![endpoints::get_post, endpoints::new_post])
        .mount("/posts", routes![endpoints::get_posts])
        .register(catchers![
            catchers::not_found,
            catchers::access_denied,
            catchers::internal_server_error
        ])
}

fn main() {
    rocket().launch();
}

fn run_migration(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = DBConn::get_one(&rocket).expect("database connection");

    match embedded_migrations::run_with_output(&*conn, &mut std::io::stdout()) {
        Ok(_) => Ok(rocket),
        Err(err) => {
            error!("failed to run database migrations: {:?}", err);
            Err(rocket)
        }
    }
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;

    #[test]
    fn get_posts() {
        let client = Client::new(rocket()).expect("wanted valid rocket instance");
        let response = client.get("/posts").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON))
    }
}
