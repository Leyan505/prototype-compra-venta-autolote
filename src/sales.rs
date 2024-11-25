use std::option;

use crate::AppState;

use bigdecimal::BigDecimal;
use chrono::Utc;
use tera::{Context, Tera};

use actix_web::{delete, dev::Path, get, http::header::FROM, post, web, HttpResponse, Responder, Result, error::InternalError};
use serde::{Deserialize, Serialize};
use sqlx::{self, PgPool};

#[derive(Serialize, Deserialize)]
pub struct Venta {
    pub id_venta: Option<i32>, //campo opcional
    pub fecha_venta: chrono::NaiveDate,
    pub matricula:String,
    pub precio_venta: BigDecimal,
    pub id_cliente: i32,
    pub id_vendedor: i32
}

#[derive(Serialize, Deserialize)]
pub struct VentaChart{
    pub record_count: Option<i64>,
    pub month: Option<i32>
}

#[get("/sales")]
pub async fn get_sales(state: web::Data<AppState>, tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    match sqlx::query_as!(
        Venta,
        r#"
         SELECT id_venta, matricula, fecha_venta, precio_venta, id_cliente, id_vendedor
        FROM venta
       "#
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(ventas) => {
            context.insert("ventas", &ventas);
            HttpResponse::Ok().body(tera.render("sales.html", &context).unwrap())
        }
        Err(_) => HttpResponse::NotFound().json("No sales found"),
    }
}

#[get("/fetch_sales")]
pub async fn fetch_sales(state: web::Data<AppState>, tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    match sqlx::query_as!(VentaChart,
        r#"
        SELECT EXTRACT(MONTH from DATE_TRUNC('month', fecha_venta))::int AS month,
        COUNT(*) AS record_count
        FROM venta
        WHERE EXTRACT(YEAR FROM fecha_venta) = EXTRACT(YEAR from CURRENT_DATE)
        GROUP BY month
        ORDER BY month;
       "#
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(ventas) => Ok(web::Json(ventas)),
        Err(err) => Err(InternalError::new(
            err.to_string(),  // Convert the error to a string or handle it accordingly
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        )), 
    }
}

#[post("/sales")]
pub async fn insert_sales(state: web::Data< AppState>, new_sales: web::Form<Venta>) -> impl Responder {
    match sqlx::query_as!( 
        Venta,
        r#"
        INSERT INTO venta (matricula, fecha_venta, precio_venta, id_cliente, id_vendedor)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        new_sales.matricula,
        new_sales.fecha_venta,
        new_sales.precio_venta,
        new_sales.id_cliente,
        new_sales.id_vendedor,
    )
    .execute(&state.db)
    .await
{
    Ok(_) => HttpResponse::SeeOther()
        .append_header(("Location", "/sales"))
        .finish(),
    Err(_) => {
        HttpResponse::InternalServerError().body("Error al insertar una venta")
    }
    }
}

#[post("/delete_sales/{id_venta}")]
pub async fn delete_sales(state: web::Data<AppState>, path: web::Path<(i32, )>) -> impl Responder{
    let id_venta = path.into_inner().0;

    let result = sqlx::query!(
        r#"
        DELETE FROM venta WHERE id_venta = $1
        "#, 
        id_venta
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => HttpResponse::SeeOther()
            .append_header(("Location", "/sales"))
            .finish(),
        Err(_) => {
            HttpResponse::InternalServerError().body("Error al eliminar la venta")
        }
        
    }
}


#[get("/sales/salesDetails/{id_venta}")]
pub async fn get_sales_details(state: web::Data<AppState>, path: web::Path<(i32, )>) -> impl Responder { 
    let id_venta = path.into_inner().0;
    match sqlx::query_as!(
        Venta,
        r#"
         SELECT id_venta, matricula, fecha_venta, precio_venta, id_cliente, id_vendedor
        FROM venta
        WHERE id_venta = $1
       "#,
        id_venta
    )   
    .fetch_all(&state.db)
    .await
    {
        Ok(venta) => Ok(web::Json(venta)),
        Err(err) => Err(InternalError::new(
            err.to_string(),  // Convert the error to a string or handle it accordingly
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        )),  // Error case
    }
}

#[post("/edit_sales/")]
pub async fn edit_sales(state: web::Data<AppState>, modified_sales: web::Form<Venta>) -> Result<HttpResponse, InternalError<String>> {
    match sqlx::query!(r#"
    UPDATE venta
    SET fecha_venta = $1, precio_venta = $2,
    id_cliente = $3, id_vendedor = $4
    WHERE id_venta = $5 
    "#,
        modified_sales.fecha_venta,
        modified_sales.precio_venta,
        modified_sales.id_cliente,
        modified_sales.id_vendedor,
        modified_sales.id_venta
      
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => Ok(HttpResponse::SeeOther().append_header(("Location", "/sales")).finish()),
        Err(err) => Err(InternalError::new(
            err.to_string(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        )),
    }


}

