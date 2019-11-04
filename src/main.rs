use actix_web::{App, HttpResponse, HttpServer, Responder};
use actix_web::get;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::fs;

mod json;

#[get("/")]
fn index3() -> impl Responder {
    let contents = fs::read_to_string("src/spells.json")
        .expect("Something went wrong reading the file");
    HttpResponse::Ok().body(contents)
}

fn read_spells_from_file<P: AsRef<Path>>(path: P) -> Result<json::types::Spell, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Spell`.
    let spell = serde_json::from_reader(reader)?;

    // Return the `Spells`.
    Ok(spell)
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
    let spells = read_spells_from_file2("test.json").unwrap();
    println!("{:#?}", spells);
    HttpServer::new(|| {
        App::new()
            .service(index3)
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
