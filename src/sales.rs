use crate::AppState;

use bigdecimal::BigDecimal;
use tera::{Context, Tera};

use actix_web::{delete, dev::Path, get, http::header::FROM, post, web, HttpResponse, Responder, Result};
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




#[post("/sales")]
pub async fn insert_sales(state: web::Data< AppState>, new_sales: web::Form<Venta>)  //manda los datos como un webform, no se como hacerlo en json
-> impl Responder {

    let result = sqlx::query!(
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
    .await;
match result {
    Ok(_) => HttpResponse::SeeOther()
        .append_header(("Location", "/sales"))
        .finish(),
    Err(err) => {
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
        Err(err) => {
            println!("Error al eliminar la venta: {:?}", err);
            HttpResponse::InternalServerError().body("Error al eliminar la venta")
        }
        
    }
}

