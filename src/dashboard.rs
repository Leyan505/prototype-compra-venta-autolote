use crate::AppState;
use actix_web::{ get, web::{self}, HttpResponse, Responder};
use tera::{Context, Tera};

#[get("/")]
pub async fn index(state: web::Data<AppState>, tera: web::Data<Tera>) -> impl Responder{
    let context = Context::new();
    HttpResponse::Ok().body(tera.render("index.html", &context).unwrap())
}