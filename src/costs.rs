use std::fmt;
use crate::AppState;
use actix_web::{error::InternalError, get, http::StatusCode, post, web::{self, Redirect}, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use bigdecimal::{BigDecimal, ToPrimitive};
use sqlx::{self, types::{chrono}, FromRow};
use tera::{Context, Tera};
use tokio::sync::OwnedRwLockMappedWriteGuard;
use rust_xlsxwriter::{Workbook,Format, XlsxError, Color};


#[derive(Serialize, Deserialize, FromRow)]
pub struct Gasto {
    pub id_gasto: Option<i32>,
    pub matricula: String,
    pub tipo_reparacion: String,
    pub monto: BigDecimal,
    pub fecha_finalizacion: chrono::NaiveDate,
    pub nombre_taller:Option<String>,
    pub direccion_taller: Option<String>,
    pub telefono_taller: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Gasto_chart {
    pub month: Option<i32>,
    pub record_count: Option<i64>
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

#[post("/edit_costs/")]
pub async fn edit_costs(state: web::Data<AppState>, modified_costs: web::Form<Gasto>) -> Result<HttpResponse, InternalError<String>> {
    match sqlx::query!(r#"
    UPDATE gasto
    SET tipo_reparacion = $1,
        monto = $2, 
        fecha_finalizacion = $3,
        nombre_taller = $4, 
        direccion_taller = $5,
        telefono_taller = $6
    WHERE matricula = $7
    "#,
        
        modified_costs.tipo_reparacion,
        modified_costs.monto,
        modified_costs.fecha_finalizacion,
        modified_costs.nombre_taller,
        modified_costs.direccion_taller,
        modified_costs.telefono_taller,
        modified_costs.matricula
        
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => Ok(HttpResponse::SeeOther().append_header(("Location", "/costs")).finish()),
        Err(err) => Err(InternalError::new(
            err.to_string(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        )),
    }
}

#[get("/fetch_costs_chart")]
pub async fn fetch_costs_chart(state: web::Data<AppState>, tera: web::Data<Tera>) -> impl Responder {

    match sqlx::query_as!(Gasto_chart,
        r#"SELECT EXTRACT(MONTH from DATE_TRUNC('month', fecha_finalizacion))::int AS month,
        COUNT(*) AS record_count
        FROM gasto
        WHERE EXTRACT(YEAR FROM fecha_finalizacion) = EXTRACT(YEAR from CURRENT_DATE)
        GROUP BY month
        ORDER BY month;
    "#)
        .fetch_all(&state.db)
        .await
    {
        Ok(gastos) => Ok(web::Json(gastos)),
        Err(err) => Err(InternalError::new(
            err.to_string(),  // Convert the error to a string or handle it accordingly
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        )), 
    }
}



#[get("/costs/export")]
pub async fn export_costs(state: web::Data<AppState>) -> impl Responder {
    let gastos_result = sqlx::query_as!(
        Gasto,
        r#"
        SELECT id_gasto, matricula, tipo_reparacion, fecha_finalizacion, monto, nombre_taller, direccion_taller, telefono_taller
        FROM gasto
        "#
    )
    .fetch_all(&state.db)
    .await;

    match gastos_result {
        Ok(gasto) => {
            match export_costs_to_xlsx(gasto, "export/archivo.xlsx").await {
                Ok(_) => {
                    match std::fs::read("export/archivo.xlsx") {
                        Ok(file_bytes) => {
                            HttpResponse::Ok()
                                .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
                                .insert_header(("Content-Disposition", "attachment; filename=sales.xlsx"))
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
            eprintln!("Error fetching sales: {}", err);
            HttpResponse::InternalServerError().body("Error fetching sales")
        }
    }
}

// genera el archivo excel
pub async fn export_costs_to_xlsx(costs: Vec<Gasto>, file_path: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let header_format = Format::new()
    .set_font_size(12.0)
    .set_bold()
    .set_background_color(Color::Green);

 

    // esrcibe los encabezados
        worksheet.write_string_with_format(0, 0, "ID Gasto", &header_format)?;
        worksheet.write_string_with_format(0, 1, "Matrícula", &header_format)?;
        worksheet.write_string_with_format(0, 2, "Fecha de Finalizacion", &header_format)?;
        worksheet.write_string_with_format(0, 2, "Tipo de Reparacion", &header_format)?;
        worksheet.write_string_with_format(0, 3, "Monto", &header_format)?;
        worksheet.write_string_with_format(0, 4, "Nombre del Taller", &header_format)?;
        worksheet.write_string_with_format(0, 5, "Direccion del Taller",&header_format)?;
        worksheet.write_string_with_format(0, 5, "Telefono del Taller",&header_format)?;
    
    //rellena los encabezados con los datos de ventas
    for (row, gasto) in costs.iter().enumerate() {
        // Escribe ID del gasto
        worksheet.write_number((row + 1) as u32, 0, gasto.id_gasto.unwrap_or(0) as f64)?;
    
        // Escribe matrícula
        worksheet.write_string((row + 1) as u32, 1, &gasto.matricula)?;
    
        // Escribe fecha de finalización como cadena
        worksheet.write_string((row + 1) as u32, 2, &gasto.fecha_finalizacion.to_string())?;
    
        // Escribe monto (manejo de BigDecimal -> f64)
        let monto = gasto.monto.to_f64().unwrap_or(0.0);
        worksheet.write_number((row + 1) as u32, 3, monto)?;
    
        // Escribe nombre del taller
        worksheet.write_string(
            (row + 1) as u32, 
            4, 
            gasto.nombre_taller.as_deref().unwrap_or("N/A")
        )?;
    
        // Escribe dirección del taller
        worksheet.write_string(
            (row + 1) as u32, 
            5, 
            gasto.direccion_taller.as_deref().unwrap_or("N/A")
        )?;
    
        // Escribe teléfono del taller
        worksheet.write_string(
            (row + 1) as u32, 
            6, 
            gasto.telefono_taller.as_deref().unwrap_or("N/A")
        )?;
    }
    

    workbook.save(file_path)?;
    Ok(())
}
