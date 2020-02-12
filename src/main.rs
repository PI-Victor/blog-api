#![feature(proc_macro_hygiene, decl_macro)]
extern crate futures;
#[macro_use]
extern crate tokio;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate config;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use clap::{App, AppSettings, Arg, SubCommand};
use config::{Config as Conf, ConfigError, Environment, File};
use rocket::config::Config as RocketConfig;
use rocket::config::Environment as RocketEnvironment;

use std::net::Ipv4Addr;
use tokio::task;
mod api;
mod http;

use http::routes as endpoints;
use http::{catchers, guards};

const VERSION: &str = "0.1.0-alpha";
const ASCIIART: &str = r#"
"#;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let matches = App::new("ceph-api")
        .settings(&[AppSettings::SubcommandRequiredElseHelp])
        .version(VERSION)
        .about(ASCIIART)
        .author("Codeflavor Org")
        .arg(
            Arg::with_name("verbosity")
                .multiple(true)
                .short("v")
                .help("application verbosity level 0-4")
                .long("verbosity"),
        )
        .subcommand(
            SubCommand::with_name("start")
                .help("start the application")
                .arg(
                    Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .value_names(&["JSON, YAML, HJSON, INI"])
                        .takes_value(true)
                        .help("path to config file")
                        .required(true),
                ),
        )
        .get_matches();

    let mut config = Configuration::default();
    if let Some(matches) = matches.subcommand_matches("start") {
        config = Configuration::new(matches.value_of("config").unwrap()).unwrap();
    }

    let log_level = match matches.occurrences_of("verbosity") {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    env_logger::Builder::from_default_env()
        .filter(Some(module_path!()), log_level)
        .init();
    info!("{}", ASCIIART);
    debug!("Loaded configuration: {:?}", config);

    task::spawn(async move {
        debug!("Mounting routes...");
        rocket::custom(
            RocketConfig::build(RocketEnvironment::Staging)
                .address(&config.bind_address.to_string())
                .port(config.bind_port)
                .finalize()
                .unwrap(),
        )
        .mount("/", routes![endpoints::index])
        .mount("/posts:postId", routes![])
        .register(catchers![catchers::not_found, catchers::access_denied])
        .launch();
        Ok::<(), std::io::Error>(())
    })
    .await
    .unwrap()
}

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub bind_address: Ipv4Addr,
    pub bind_port: u16,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1".parse::<Ipv4Addr>().unwrap(),
            bind_port: 8080,
        }
    }
}

impl Configuration {
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        let mut c = Conf::new();
        c.merge(File::with_name(path))?;
        c.merge(Environment::with_prefix("BLOG_API"))?;
        c.try_into()
    }
}
