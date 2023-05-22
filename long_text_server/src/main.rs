// src/main.rs
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web,Result, post};
use diesel::RunQueryDsl;
use schema::long_text;

use self::schema::long_text::dsl::*;
use self::models::*;
use diesel::prelude::*;
use long_text_server::*;
mod models;
mod schema;

fn show_text() -> Vec<LongText> {
    let connection = &mut establish_connection();
    let results = long_text.load::<LongText>(connection).expect("nooo");
    results
}

fn insert_text(text : LongTextInsertor) -> LongText {
    let connection = &mut establish_connection();
    diesel::insert_into(long_text::table)
        .values(&text)
        .get_result(connection)
        .expect("Coudln't save")
}


#[get("/")]
async fn index() -> impl Responder {
    show_text();
    HttpResponse::Ok().body("Hello world!")
}

#[get("/texts/all")]
async fn get_all() -> Result<impl Responder> {
    let results = show_text();
    Ok(web::Json(results))
}

#[post("/texts")]
async fn post_text(text : web::Json<LongTextInsertor>) -> Result<impl Responder> {
    let actual_text = text.into_inner();
     Ok(web::Json(insert_text(actual_text)))
    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODOS
    // connection pooling
    // add methods here so I can pass the connection
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(get_all)
            .service(post_text)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}



/*
 * insert into long_text (difficulty, source, body) values ("easy", "typeracer", "The quick brown fox jumped over the lazy brown dog")
 * */
