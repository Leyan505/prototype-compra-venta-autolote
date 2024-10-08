use std::string;

use crate::AppState;
use actix_web::{
    get,
    http::header::FROM,
    post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::{self, types::chrono, FromRow};

#[derive(Serialize, FromRow)]
struct Vehiculo {
    nro_chasis: String,
    matricula: String,
    model: String,
    marca: String,
    color: String,
    anio: i32,
}

#[derive(Serialize, FromRow)]
struct Compra {
    id_compra: i32,
    fecha_compra: chrono::DateTime<chrono::Utc>,
    precio_compra: f64,
    gastos_viaje: f64,
    nro_chasis: String,
}

#[get("/vehiculos")]
pub async fn fetch_vehicles(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Vehiculo>(
        "SELECT nro_chasis, matricula, modelo, marca, color, anio FROM vehiculos",
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(vehiculos) => HttpResponse::Ok().json(vehiculos),
        Err(_) => HttpResponse::NotFound().json("No vehicles found"),
    }
}
