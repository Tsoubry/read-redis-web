use actix_utils::future::ok;
use actix_web::{middleware, web, App, HttpServer};

use clap::Parser;
use log::info;

use crate::config::AppConfig;
use crate::routes::get_from_key;

mod config;
mod error;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let options = AppConfig::parse();

    info!("Server starting on {}:{}", &options.host, &options.port);

    let client = redis::Client::open(options.get_redis_uri()).unwrap();

    HttpServer::new(move || {
        App::new()
            .data_factory(|| ok::<_, ()>(AppConfig::parse()))
            .app_data(web::Data::new(client.clone()))
            .wrap(middleware::Logger::default())
            .service(get_from_key)
    })
    .bind((options.host, options.port))?
    .run()
    .await
}
