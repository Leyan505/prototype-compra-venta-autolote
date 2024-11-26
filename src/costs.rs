use std::fmt;
use crate::AppState;
use actix_web::{error::InternalError, get, http::StatusCode, post, web::{self, Redirect}, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{self, types::{chrono, BigDecimal}, FromRow};
use tera::{Context, Tera};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Gasto {
    pub id_gasto: i32,
    pub matricula: String,
    pub tipo_reparacion: String,
    pub monto: BigDecimal,
    pub fecha_finalizacion: chrono::NaiveDate,
    pub nombre_taller: String,
    pub direccion_taller: Option<String>,
    pub telefono_taller: Option<String>,
}

impl fmt::Display for Gasto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} - {} - {} ({})",
            self.matricula, self.tipo_reparacion, self.monto, self.fecha_finalizacion
        )
    }
}

#[get("/costs")]
pub async fn fetch_costs(state: web::Data<AppState>, tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();

    match sqlx::query_as!(Gasto,
        r#"SELECT id_gasto, matricula, tipo_reparacion, monto, fecha_finalizacion, nombre_taller, direccion_taller, telefono_taller FROM gasto"#)
        .fetch_all(&state.db)
        .await
    {
        Ok(gastos) => {
            context.insert("gastos", &gastos);
            HttpResponse::Ok()
                .body(tera.render("costs.html", &context).unwrap())
        }
        Err(_) => HttpResponse::NotFound().json("No costs found"),
    }
}

#[post("/costs")]
pub async fn insert_costs(state: web::Data<AppState>, new_cost: web::Form<Gasto>) -> impl Responder {
    match sqlx::query!(
        r#"
        INSERT INTO gasto (matricula, tipo_reparacion, monto, fecha_finalizacion, nombre_taller, direccion_taller, telefono_taller)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        new_cost.matricula,
        new_cost.tipo_reparacion,
        new_cost.monto,
        new_cost.fecha_finalizacion,
        new_cost.nombre_taller,
        new_cost.direccion_taller,
        new_cost.telefono_taller
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => HttpResponse::SeeOther()
            .append_header(("Location", "/costs"))
            .finish(),
        Err(_) => HttpResponse::InternalServerError().body("Error al insertar gasto"),
    }
}

#[post("/delete_costs/{id}")]
pub async fn delete_costs(
    state: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> Result<Redirect, InternalError<String>> {
    let id = path.into_inner().0;
    match sqlx::query!(
        r#"
        DELETE FROM gasto
        WHERE id_gasto = $1
        "#,
        id,
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => Ok(Redirect::to("/costs").see_other()),
        Err(err) => Err(InternalError::new(
            err.to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

#[get("/costs/details/{id}")]
pub async fn get_cost(
    state: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> Result<impl Responder, InternalError<String>> {
    let id = path.into_inner().0;
    match sqlx::query_as!(Gasto,
        r#"
        SELECT id_gasto, matricula, tipo_reparacion, monto, fecha_finalizacion, nombre_taller, direccion_taller, telefono_taller
        FROM gasto
        WHERE id_gasto = $1
        "#,
        id
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(gasto) => Ok(web::Json(gasto)),
        Err(err) => Err(InternalError::new(
            err.to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}