use crate::pg_pool_handler;
use actix_web::{
    get,
    web,
    HttpResponse,
    Responder,
};
use log::debug;
use splitterrust_db::get_spell_by_name as get_spell;
use splitterrust_db::get_spell_like_name as get_spells;
use splitterrust_db::models::spell_schools::Spell as SpellSchools;
use splitterrust_db::PgPool;

/// The index of the server.
///
/// Should be changed any time later to something useful.
#[get("/")]
pub fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello there, Gerneral Kenobi!")
}

/// Returns a HTTP response, which contains a list of spells on success.
///
/// # Arguments
///
/// * `name`: Derived from the url with {name}
/// * `pool`: The database pool which will be used for the database
///           communication
#[get("/spell/{name}")]
pub fn get_spell_by_name(name: web::Path<String>, pool: web::Data<PgPool>) -> impl Responder {
    let pg_pool = pg_pool_handler(pool).unwrap();
    let spell_name = name.into_inner();
    debug!("get_spell_by_name(name: {}", spell_name);

    if spell_name.contains("%") {
        debug!("spell_name contains %, searching with no limit");
        let result = get_spells(&spell_name, &pg_pool);

        return match result.len() {
            0 => HttpResponse::NotFound().body("No spell found"),
            _ => HttpResponse::Ok().json(
                result
                    .iter()
                    .map(|spell_schools| SpellSchools::from_left_join(spell_schools.clone()))
                    .collect::<Vec<_>>(),
            ),
        };
    } else {
        debug!("spell_name contains no %, searching for exact match");
        return match get_spell(&spell_name, &pg_pool) {
            None => HttpResponse::NotFound().body("No spell found"),
            Some(result) => HttpResponse::Ok().json(vec![SpellSchools::from_left_join(result)]),
        };
    }
}
