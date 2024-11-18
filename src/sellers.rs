use crate::AppState;

use tera::{Context, Tera};

use actix_web::{error::InternalError, get, http::header::FROM, post, web, HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};
use sqlx::{self};

#[derive(Serialize, Deserialize)]
pub struct Vendedor {
    pub id_vendedor: Option<i32>, //campo opcional
    pub nombre: String,
    pub apellido: String,
    pub cedula: String,
}

#[get("/sellers")]
pub async fn obtain_sellers(state: web::Data<AppState>, tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    match sqlx::query_as!(
        Vendedor,
        r#"
         SELECT id_vendedor, nombre, apellido, cedula
        FROM vendedor
       "#
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(vendedores) => {
            context.insert("vendedores", &vendedores);
            HttpResponse::Ok().body(tera.render("seller.html", &context).unwrap())
        }
        Err(_) => HttpResponse::NotFound().json("No sellers found"),
    }
}




#[post("/sellers")]
pub async fn post_sellers(state: web::Data< AppState>, new_seller: web::Form<Vendedor>)  //manda los datos como un webform, no se como hacerlo en json
-> impl Responder {

    let result = sqlx::query!(
        r#"
        INSERT INTO vendedor (nombre, apellido, cedula)
        VALUES ($1, $2, $3)
        "#,
        new_seller.nombre,
        new_seller.apellido,
        new_seller.cedula
    )
    .execute(&state.db)
    .await;
match result {
    Ok(_) => HttpResponse::SeeOther()
        .append_header(("Location", "/sellers"))
        .finish(),
        Err(err) => InternalError::new(err.to_string(), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        .error_response(),
    
}
}

#[post("/delete_sellers/{id_vendedor}")]
pub async fn delete_sellers(state: web::Data<AppState>, path: web::Path<(i32, )>) -> impl Responder{
    let id_vendedor = path.into_inner().0;

    let result = sqlx::query!(
        r#"
        DELETE FROM vendedor WHERE id_vendedor = $1
        "#, 
        id_vendedor
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => HttpResponse::SeeOther()
            .append_header(("Location", "/sellers"))
            .finish(),
        Err(err) => {
            println!("Error al eliminar la venta: {:?}, se viola una fk", err);
            HttpResponse::InternalServerError().body("Error al eliminar la venta")
        }
        
    }
}

