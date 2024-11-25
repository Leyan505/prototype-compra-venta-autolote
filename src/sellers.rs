use crate::AppState;
use tera::{Context, Tera};
use actix_web::{error::InternalError, get, post, web, HttpResponse, Responder, ResponseError};
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
pub async fn get_sellers(state: web::Data<AppState>, tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    match sqlx::query_as!(
        Vendedor,
        r#"
         SELECT id_vendedor, nombre, apellido, cedula
        FROM vendedor WHERE estado = 'ACTIVO' 
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
pub async fn insert_sellers(state: web::Data< AppState>, new_seller: web::Form<Vendedor>)  //manda los datos como un webform, no se como hacerlo en json
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
        UPDATE vendedor SET estado = 'OUT' WHERE id_vendedor = $1
        "#, 
        id_vendedor
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


#[get("/sellers/sellerDetails/{id_vendedor}")]
pub async fn get_seller_details(state: web::Data<AppState>, path: web::Path<(i32, )>) -> impl Responder { 
    let id_vendedor = path.into_inner().0;
    match sqlx::query_as!(
        Vendedor,
        r#"
        SELECT id_vendedor ,nombre, apellido,cedula
        FROM vendedor
        WHERE id_vendedor = $1
       "#,
        id_vendedor
    )   
    .fetch_all(&state.db)
    .await
    {
        Ok(vendedor) => Ok(web::Json(vendedor)),
        Err(err) => Err(InternalError::new(
            err.to_string(),  // Convert the error to a string or handle it accordingly
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        )),  // Error case
    }
}



#[post("/edit_sellers/")]
pub async fn edit_sellers(state: web::Data<AppState>, modified_sellers: web::Form<Vendedor>) -> Result<HttpResponse, InternalError<String>> {
    match sqlx::query!(r#"
    UPDATE vendedor
    SET nombre= $1, apellido = $2,
    cedula = $3
    WHERE id_vendedor = $4 
    "#,
        modified_sellers.nombre,
        modified_sellers.apellido,
        modified_sellers.cedula,
        modified_sellers.id_vendedor,
        
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => Ok(HttpResponse::SeeOther().append_header(("Location", "/sellers")).finish()),
        Err(err) => Err(InternalError::new(
            err.to_string(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        )),
    }


}

