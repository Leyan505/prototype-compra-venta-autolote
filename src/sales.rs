use crate::AppState;

use bigdecimal::BigDecimal;
use tera::{Context, Tera};

use actix_web::{get, http::header::FROM, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{self};

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




// #[post("/sales")]
// pub async fn in_sales(state: web::Data< AppState>, new_seller: web::Form<Ventasr>)  //manda los datos como un webform, no se como hacerlo en json
// -> impl Responder {

//     let result = sqlx::query!(
//         r#"
//         INSERT INTO venta (matricula, fecha_venta, precio_venta, id_cliente, id_vendedor)
//         VALUES ($1, $2, $3, $4, $5)
//         "#,
//         new_sales.matricula,
//         new_sales.fecha_venta,
//         new_sales.precio_venta,
//         new_slaes.id_cliente,
//         new_slaes.id_vendedor,
//     )
//     .execute(&state.db)
//     .await;
// match result {
//     Ok(_) => HttpResponse::SeeOther()
//         .append_header(("Location", "/sales"))
//         .finish(),
//     Err(err) => {
//         HttpResponse::InternalServerError().body("Error al insertar una venta")
//     }
// }
// }

