use std::string;
use std::fmt;
use crate::AppState;
use actix_web::{error::InternalError, get, http::{header::FROM, StatusCode}, post, web::{self, Json, Redirect}, HttpResponse, Responder, ResponseError};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{self, types::{chrono, BigDecimal}, FromRow};
use tera::{Context, Tera};

#[get("/")]
pub async fn index(state: web::Data<AppState>, tera: web::Data<Tera>) -> impl Responder{
    let mut context = Context::new();
    HttpResponse::Ok().body(tera.render("index.html", &context).unwrap())
}