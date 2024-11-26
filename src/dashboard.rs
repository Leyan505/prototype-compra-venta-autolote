use crate::AppState;
use actix_web::{ get, web::{self}, HttpResponse, Responder};
use tera::{Context, Tera};

#[derive(Serialize, Deserialize)]
pub struct ganancias{
    pub total_difference: Option<BigDecimal>
}

#[derive(Serialize, Deserialize)]
pub struct compras{
    pub total: Option<BigDecimal>
}

#[derive(Serialize, Deserialize)]
pub struct ventas{
    pub total: Option<BigDecimal>
}

#[derive(Serialize, Deserialize)]
pub struct vehiculosVendidos{
    pub total: Option<i64>
}


#[get("/")]
pub async fn index(state: web::Data<AppState>, tera: web::Data<Tera>) -> impl Responder{
    let context = Context::new();
    HttpResponse::Ok().body(tera.render("index.html", &context).unwrap())
}

#[get("/fetch_earnings")]
pub async fn fetch_earnings(state: web::Data<AppState>, tera: web::Data<Tera>) -> impl Responder {

    match sqlx::query_as!(ganancias,
        r#"
        SELECT 
        ( 
            (SELECT SUM(monto) FROM gasto where EXTRACT(MONTH from fecha_finalizacion) = EXTRACT(MONTH from CURRENT_DATE)) 
            + 
            (SELECT SUM(precio_compra) FROM vehiculo where EXTRACT(MONTH from fecha_compra) = EXTRACT(MONTH from CURRENT_DATE)) 
            - 
            (SELECT SUM(precio_venta) FROM venta where EXTRACT(MONTH from fecha_venta) = EXTRACT(MONTH from CURRENT_DATE))
        ) 
        AS total_difference
       "#
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(ganancias) => Ok(web::Json(ganancias)),
        Err(err) => Err(InternalError::new(
            err.to_string(),  // Convert the error to a string or handle it accordingly
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        )), 
    }
}

#[get("/fetch_buys")]
pub async fn fetch_buys(state: web::Data<AppState>, tera: web::Data<Tera>) -> impl Responder {

    match sqlx::query_as!(compras,
        r#"
        SELECT SUM(precio_compra) as total 
        FROM vehiculo where EXTRACT(MONTH from fecha_compra) = EXTRACT(MONTH from CURRENT_DATE);
       "#
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(compras) => Ok(web::Json(compras)),
        Err(err) => Err(InternalError::new(
            err.to_string(),  // Convert the error to a string or handle it accordingly
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        )), 
    }
}

#[get("/fetch_sells")]
pub async fn fetch_sells(state: web::Data<AppState>, tera: web::Data<Tera>) -> impl Responder {

    match sqlx::query_as!(ventas,
        r#"
        SELECT SUM(precio_venta) as total FROM venta 
        where EXTRACT(MONTH from fecha_venta) = EXTRACT(MONTH from CURRENT_DATE)
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

#[get("/fetch_vehicles_sold")]
pub async fn fetch_vehicles_sold(state: web::Data<AppState>, tera: web::Data<Tera>) -> impl Responder {

    match sqlx::query_as!(vehiculosVendidos,
        r#"
        SELECT COUNT(*) as total FROM venta
        WHERE EXTRACT(MONTH from fecha_venta) = EXTRACT(MONTH from CURRENT_DATE)
       "#
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(vehiculos_vendidos) => Ok(web::Json(vehiculos_vendidos)),
        Err(err) => Err(InternalError::new(
            err.to_string(),  // Convert the error to a string or handle it accordingly
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        )), 
    }
}