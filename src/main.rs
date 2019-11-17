use actix_web::{HttpServer, App};

mod rest_api;
use rest_api::entrypoints::index;
use rest_api::entrypoints::get_spell_by_name as get_spell;

fn main() {
    http()
}

fn http() {
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(get_spell)
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
