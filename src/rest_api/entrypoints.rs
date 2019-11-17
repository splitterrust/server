use actix_web::{HttpResponse, Responder, get, web};
use splitterrust_db::get_spell_by_name as get_spell;

#[get("/")]
pub fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello there, Gerneral Kenobi!")
}

#[get("/spell/{name}")]
pub fn get_spell_by_name(name: web::Path<String>) -> impl Responder {
    let spell_name = name.into_inner();
    match get_spell(&spell_name) {
        Some(spell) => HttpResponse::Ok().body(spell.to_string()),
        None        => HttpResponse::Ok().body("Spell not found")
    }
}