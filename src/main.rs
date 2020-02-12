use actix_web::{App, HttpServer};

mod rest_api;
use rest_api::entrypoints::get_spell_by_name as get_spell;
use rest_api::entrypoints::index;

use dotenv::dotenv;
use log::info;

fn main() {
    dotenv().ok();
    //std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    info!("Starting server");
    http()
}

fn http() {
    HttpServer::new(move || App::new().service(index).service(get_spell))
        .bind("0.0.0.0:8088")
        .unwrap()
        .run()
        .unwrap();
}
