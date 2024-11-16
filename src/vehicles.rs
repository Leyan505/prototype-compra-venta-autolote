use std::string;

use crate::AppState;
use actix_web::{error::InternalError, get, http::{header::FROM, StatusCode}, post, web::{self, Json, Redirect}, HttpResponse, Responder, ResponseError};
use serde::{de::Error, Deserialize, Serialize};
use sqlx::{
    self, error::DatabaseError, types::{chrono, BigDecimal}, FromRow
};
use tera::{Context, Tera};

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "state")]
pub enum state {
    DISPONIBLE,
    ENPROCESO,
    VENDIDO,
    RESERVADO,
}

#[derive(Serialize, Deserialize, FromRow)]
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

#[get("/vehicles")]
pub async fn fetch_vehicles(state: web::Data<AppState>, tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();

    match sqlx::query_as!(Vehiculo,
        r#"SELECT nro_chasis, matricula, modelo, marca, color, anio, fecha_compra, precio_compra, estado AS "estado!: state" FROM vehiculo"#)
        .fetch_all(&state.db)
        .await
    {
        Ok(vehiculos) => {
            context.insert("vehiculos", &vehiculos);
            HttpResponse::Ok().body(tera.render("vehicles.html", &context).unwrap())
        }
        Err(_) => HttpResponse::NotFound().json("No vehicles found"),
    }
}

#[post("/vehicles")]
pub async fn insert_vehicles(state: web::Data<AppState>, new_vehicle: web::Form<Vehiculo>) -> impl Responder {
    let mut context = Context::new();

    match sqlx::query_as!(Vehiculo,
    r#"
        INSERT INTO vehiculo (nro_chasis, matricula, modelo, marca, color, anio, fecha_compra, precio_compra, estado)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
    "#,
        new_vehicle.nro_chasis,
        new_vehicle.matricula,
        new_vehicle.modelo,
        new_vehicle.marca,
        new_vehicle.color,
        new_vehicle.anio,
        new_vehicle.fecha_compra,
        new_vehicle.precio_compra,
        new_vehicle.estado as _ 
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => HttpResponse::SeeOther()
            .append_header(("Location", "/vehiculos"))
            .finish(),
        Err(_) => {
            HttpResponse::InternalServerError().body("Error al insertar vehiculo")
        }
    }
}

#[post("/delete_vehicles/{id}")]
pub async fn delete_vehicles(state: web::Data<AppState>, path: web::Path<(String, )>) -> Result <actix_web::web::Redirect, InternalError<String>>{
    let id = path.into_inner().0;
    match sqlx::query!(
    r#"
        DELETE FROM vehiculo
        WHERE matricula = $1
    "#,
        id,
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => Ok(Redirect::to("/vehicles").see_other()),
        Err(err) => Err(InternalError::new(
            err.to_string(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        )),
    }
}

#[get("/vehicles/vehicleDetails/{id}")]
pub async fn get_vehicle(state: web::Data<AppState>, path: web::Path<(String, )>) -> Result<impl Responder, InternalError<String>> { 
    let id = path.into_inner().0;
    match sqlx::query_as!(Vehiculo,
    r#"
    SELECT nro_chasis, matricula, modelo, marca, color, anio, fecha_compra, precio_compra, estado AS "estado!: state"
    FROM vehiculo
    WHERE matricula = $1
    "#,
        id
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(vehiculos) => Ok(web::Json(vehiculos)),
        Err(err) => Err(InternalError::new(
            err.to_string(),  // Convert the error to a string or handle it accordingly
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        )),  // Error case
    }
}
