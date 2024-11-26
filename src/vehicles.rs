use std::string;
use bigdecimal::{BigDecimal, ToPrimitive};
use derive_more::derive::Display;
use std::fmt;
use crate::AppState;
use actix_web::{error::InternalError, get, http::{header::FROM, StatusCode}, post, web::{self, Json, Redirect}, HttpResponse, Responder, ResponseError};
use serde::{de::Error, Deserialize, Serialize};
use sqlx::{self, error::DatabaseError, types::{chrono}, FromRow
};
use rust_xlsxwriter::{Workbook,Format, XlsxError, Color};

use tera::{Context, Tera};

#[derive(Serialize, Deserialize, sqlx::Type, Clone, Copy, Display)]
#[sqlx(type_name = "state")]
pub enum state {
    DISPONIBLE,
    ENPROCESO,
    VENDIDO,
    RESERVADO,
    ELIMINADO
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

#[derive(Serialize, Deserialize)]
pub struct VehiculoChart{
    pub record_count: Option<i64>,
    pub month: Option<i32>
}

impl fmt::Display for Vehiculo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} ({})", self.marca, self.modelo, self.anio)
    }
}

#[get("/vehicles")]
pub async fn fetch_vehicles(state: web::Data<AppState>, tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();

    match sqlx::query_as!(Vehiculo,
        r#"SELECT nro_chasis, matricula, modelo, marca, color, anio, fecha_compra, precio_compra, estado as "estado!: state"
        FROM vehiculo
        WHERE estado != 'ELIMINADO'"#)
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

#[get("/fetch_vehicles_chart")]
pub async fn fetch_vehicles_chart(state: web::Data<AppState>, tera: web::Data<Tera>) -> impl Responder {

    match sqlx::query_as!(VehiculoChart,
        r#"
        SELECT EXTRACT(MONTH from DATE_TRUNC('month', fecha_compra))::int AS month,
        COUNT(*) AS record_count
        FROM vehiculo
        WHERE EXTRACT(YEAR FROM fecha_compra) = EXTRACT(YEAR from CURRENT_DATE)
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

#[post("/vehicles")]
pub async fn insert_vehicles(state: web::Data<AppState>, new_vehicle: web::Form<Vehiculo>) -> impl Responder {

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
        new_vehicle.estado as state 
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => HttpResponse::SeeOther()
            .append_header(("Location", "/vehicles"))
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
    UPDATE vehiculo
    SET estado = 'ELIMINADO'
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

#[post("/edit_vehicles/")]
pub async fn edit_vehicles(state: web::Data<AppState>, modified_vehicle: web::Form<Vehiculo>) -> Result<HttpResponse, InternalError<String>> {
    match sqlx::query!(r#"
    UPDATE vehiculo
    SET marca = $1, color = $2, anio = $3,
    fecha_compra = $4, precio_compra = $5,
    estado = $6, modelo = $7
    WHERE matricula = $8
    "#,
        modified_vehicle.marca,
        modified_vehicle.color,
        modified_vehicle.anio,
        modified_vehicle.fecha_compra,
        modified_vehicle.precio_compra,
        modified_vehicle.estado as state,
        modified_vehicle.modelo,
        modified_vehicle.matricula
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

// test export to xls
#[get("/vehicles/export")]
pub async fn export_vehicles(state: web::Data<AppState>) -> impl Responder {
    let vehiculos_result = sqlx::query_as!(
        Vehiculo,
        r#"
        SELECT nro_chasis, matricula, modelo, marca, color, anio, fecha_compra, precio_compra, estado as "estado!: state"
        FROM vehiculo
        WHERE estado != 'ELIMINADO'"#
    )
    .fetch_all(&state.db)
    .await;

    match vehiculos_result {
        Ok(vehiculos) => {
            match export_vehicles_to_xlsx(vehiculos, "export/archivo.xlsx").await {
                Ok(_) => {
                    match std::fs::read("export/archivo.xlsx") {
                        Ok(file_bytes) => {
                            HttpResponse::Ok()
                                .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
                                .insert_header(("Content-Disposition", "attachment; filename=vehicles.xlsx"))
                                .body(file_bytes)
                        },
                        Err(err) => {
                            eprintln!("Error reading Excel file: {}", err);
                            HttpResponse::InternalServerError().body("Error reading Excel file")
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error creating Excel file: {}", err);
                    HttpResponse::InternalServerError().body("Error creating Excel file")
                }
            }
        }
        Err(err) => {
            eprintln!("Error fetching vehicles: {}", err);
            HttpResponse::InternalServerError().body("Error fetching sales")
        }
    }
}

// genera el archivo excel
pub async fn export_vehicles_to_xlsx(sales: Vec<Vehiculo>, file_path: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let header_format = Format::new()
    .set_font_size(12.0)
    .set_bold()
    .set_background_color(Color::Green);

 

    // esrcibe los encabezados
        worksheet.write_string_with_format(0, 0, "Nro chasis", &header_format)?;
        worksheet.write_string_with_format(0, 1, "Matrícula", &header_format)?;
        worksheet.write_string_with_format(0, 2, "Modelo", &header_format)?;
        worksheet.write_string_with_format(0, 3, "Marca", &header_format)?;
        worksheet.write_string_with_format(0, 4, "color", &header_format)?;
        worksheet.write_string_with_format(0, 5, "anio",&header_format)?;
        worksheet.write_string_with_format(0, 6, "fecha_compra",&header_format)?;
        worksheet.write_string_with_format(0, 7, "precio_compra",&header_format)?;
        worksheet.write_string_with_format(0, 8, "estado",&header_format)?;
    
    //rellena los encabezados con los datos de ventas
    for (row, vehiculo) in sales.iter().enumerate() {
        worksheet.write_string((row + 1) as u32, 0, &vehiculo.nro_chasis)?;
        worksheet.write_string((row + 1) as u32, 1, &vehiculo.matricula)?;
        worksheet.write_string((row + 1) as u32, 2, &vehiculo.modelo)?;
        worksheet.write_string((row + 1) as u32, 3, &vehiculo.marca)?;
        worksheet.write_string((row + 1) as u32, 4, &vehiculo.color.clone().unwrap())?;        
        worksheet.write_number((row + 1) as u32, 5, vehiculo.anio)?;        
        worksheet.write_string((row + 1) as u32, 6, &vehiculo.fecha_compra.to_string())?;        
        worksheet.write_number((row + 1) as u32, 7, vehiculo.precio_compra.to_f64().unwrap_or(0.0))?;
        worksheet.write_string((row + 1) as u32, 8, &vehiculo.estado.to_string())?;        

    }

    workbook.save(file_path)?;
    Ok(())
}
//fin teste