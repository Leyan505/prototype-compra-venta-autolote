use crate::AppState;
use std::string;
use tera::{Context, Tera};

use actix_web::{get, http::header::FROM, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{
    self,
    types::{chrono, BigDecimal},
    FromRow,
};

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "state")]
pub enum state {
    DISPONIBLE,
    ENPROCESO,
    VENDIDO,
    RESERVADO,
}

#[derive(Serialize, FromRow)]
struct Vehiculo {
    nro_chasis: String,
    matricula: String,
    modelo: String,
    marca: String,
    color: Option<String>,
    anio: i32,
    fecha_compra: chrono::NaiveDate,
    precio_compra: BigDecimal,
    estado: state,
}

#[get("/vehiculos")]
pub async fn fetch_vehicles(state: web::Data<AppState>, tera: web::Data<Tera>) -> impl Responder {
    let mut Context = Context::new();

    match sqlx::query_as!(Vehiculo,
        r#"SELECT nro_chasis, matricula, modelo, marca, color, anio, fecha_compra, precio_compra, estado AS "estado!: state" FROM vehiculo"#)
        .fetch_all(&state.db)
        .await
    {
        Ok(vehiculos) => {
            Context.insert("vehiculos", &vehiculos);
            HttpResponse::Ok().body(tera.render("index.html", &Context).unwrap())
        }
        Err(_) => HttpResponse::NotFound().json("No vehicles found"),
    }
}
