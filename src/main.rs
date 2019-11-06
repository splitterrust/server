extern crate rand;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::get;
use std::error::Error;
use std::fs;
use rand::seq::SliceRandom;
use serde_json::json;

mod json;

struct AppState {
    spells: Vec<json::types::Spell>,
}

#[get("/")]
fn index3(data: web::Data<AppState>) -> impl Responder {
    let spell = match data.spells.choose(&mut rand::thread_rng()) {
        Some(spell) => spell,
        None => panic!("Failed to get from vec"),
    };
    let j = json!(spell);
    HttpResponse::Ok().body(serde_json::to_string_pretty(&j).unwrap())
}

fn read_spells_from_file2(path: &str) -> Result<Vec<json::types::Spell>, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let contents = fs::read_to_string(path)
        .expect("Failed to read File");

    // Read the JSON contents of the file as an instance of `Spell`.
    let spells: Vec<json::types::Spell> = serde_json::from_str(&contents)?;

    // Return the `Spells`.
    Ok(spells)
}

fn main() {
    let spells = read_spells_from_file2("./src/test.json").unwrap();
    http(spells)
}

fn http(spells: Vec<json::types::Spell>) {
    HttpServer::new(move || {
        let _spells = spells.clone();
        App::new()
            .data(AppState {
                spells: _spells
            })
            .service(index3)
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}