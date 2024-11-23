use std::string;
use std::fmt;
use crate::AppState;
use actix_web::{error::InternalError, get, http::{header::FROM, StatusCode}, post, web::{self, Json, Redirect}, HttpResponse, Responder, ResponseError};
use serde::{de::Error, Deserialize, Serialize};
use sqlx::{self, error::DatabaseError, types::{chrono, BigDecimal}, FromRow
};
use tera::{Context, Tera};

#[derive(Serialize, Deserialize, FromRow)]
struct Cliente {
    id_cliente: i32,
    nombre: String,
    apellido: String,
    cedula: String,
}
impl fmt::Display for Cliente {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.nombre, self.apellido)
    }
}
#[get("/clients")]
pub async fn fetch_clients(state: web::Data<AppState>, tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();

    match sqlx::query_as!(Cliente,
        r#"SELECT id_cliente, nombre, apellido, cedula FROM cliente"#)
        .fetch_all(&state.db)
        .await
    {
        Ok(clientes) => {
            context.insert("clientes", &clientes);
            HttpResponse::Ok().body(tera.render("clients.html", &context).unwrap())
        }
        Err(_) => HttpResponse::NotFound().json("No clients found"),
    }
}

#[post("/clients")]
pub async fn insert_client(state: web::Data<AppState>, new_client: web::Form<Cliente>) -> impl Responder {
    let mut context = Context::new();

    match sqlx::query_as!(Cliente,
    r#"
        INSERT INTO cliente (id_cliente, nombre, apellido, cedula)
        VALUES ($1, $2, $3, $4)
    "#,
        new_client.id_cliente,
        new_client.nombre,
        new_client.apellido,
        new_client.cedula,
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => HttpResponse::SeeOther()
            .append_header(("Location", "/clients"))
            .finish(),
        Err(_) => {
            HttpResponse::InternalServerError().body("Error al insertar vehiculo")
        }
    }
}

#[post("/delete_client/{id}")]
pub async fn delete_client(state: web::Data<AppState>, path: web::Path<(i32, )>) -> Result <actix_web::web::Redirect, InternalError<String>>{
    let id = path.into_inner().0;
    match sqlx::query!(
    r#"
        DELETE FROM cliente
        WHERE id_cliente = $1
    "#,
        id,
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => Ok(Redirect::to("/clients").see_other()),
        Err(err) => Err(InternalError::new(
            err.to_string(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        )),
    }
}

#[get("/clients/clientDetails/{id}")]
pub async fn get_client(state: web::Data<AppState>, path: web::Path<(String, )>) -> Result<impl Responder, InternalError<String>> { 
    let id: i32 = path.into_inner().0.parse().expect("failed to parse string to integer");
    match sqlx::query_as!(Cliente,
    r#"
    SELECT id_cliente, nombre, apellido, cedula
    FROM cliente
    WHERE id_cliente = $1
    "#,
        id
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(client) => Ok(web::Json(client)),
        Err(err) => Err(InternalError::new(
            err.to_string(),  // Convert the error to a string or handle it accordingly
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        )),  // Error case
    }
}

#[post("/edit_client/")]
pub async fn edit_client(state: web::Data<AppState>, modified_client: web::Form<Cliente>) -> Result<HttpResponse, InternalError<String>> {
    match sqlx::query!(r#"
    UPDATE cliente
    SET nombre = $1, apellido = $2, cedula = $3
    WHERE id_cliente = $4
    "#,
        modified_client.nombre,
        modified_client.apellido,
        modified_client.cedula,
        modified_client.id_cliente
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => Ok(HttpResponse::SeeOther().append_header(("Location", "/vehicles")).finish()),
        Err(err) => Err(InternalError::new(
            err.to_string(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        )),
    }
}
