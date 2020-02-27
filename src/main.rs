use actix_web::{
    web,
    App,
    HttpResponse,
    HttpServer,
};

mod rest_api;
use rest_api::entrypoints::get_spell_by_name as get_spell;
use rest_api::entrypoints::index;
use splitterrust_db::{
    establish_connection,
    PgPool,
    PgPooledConnection,
};

use dotenv::dotenv;
use log::info;

fn main() {
    dotenv().ok();
    //std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    info!("Starting server");
    http()
}

/// Serves the http server.
///
/// Curretnly port 8088 is used.
fn http() {
    HttpServer::new(move || {
        App::new()
            .data(establish_connection())
            .service(index)
            .service(get_spell)
    })
    .bind("0.0.0.0:8088")
    .unwrap()
    .run()
    .unwrap();
}

/// Extracts the PgPool from the actix data and returns it.
///
/// If the pool could not be extracted, a HttpResponse is returned.
///
/// # Arguments
///
/// * `pool`: The actix data from wich to extract.
fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
