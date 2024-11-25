use std::option;

use crate:: AppState;

use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::Utc;
use tera::{Context, Tera};

use actix_web::{ get, post, web, HttpResponse, Responder, Result, error::InternalError};
use serde::{Deserialize, Serialize};
use sqlx;  
use rust_xlsxwriter::{Workbook,Format, XlsxError, Color};

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



// test export to xls
#[get("/sales/export")]
pub async fn export_sales(state: web::Data<AppState>) -> impl Responder {
    let ventas_result = sqlx::query_as!(
        Venta,
        r#"
        SELECT id_venta, matricula, fecha_venta, precio_venta, id_cliente, id_vendedor
        FROM venta
        "#
    )
    .fetch_all(&state.db)
    .await;

    match ventas_result {
        Ok(ventas) => {
            match export_sales_to_xlsx(ventas, "export/archivo.xlsx").await {
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
pub async fn export_sales_to_xlsx(sales: Vec<Venta>, file_path: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let header_format = Format::new()
    .set_font_size(12.0)
    .set_bold()
    .set_background_color(Color::Green);

 

    // esrcibe los encabezados
        worksheet.write_string_with_format(0, 0, "ID Venta", &header_format)?;
        worksheet.write_string_with_format(0, 1, "Matrícula", &header_format)?;
        worksheet.write_string_with_format(0, 2, "Fecha Venta", &header_format)?;
        worksheet.write_string_with_format(0, 3, "Precio Venta", &header_format)?;
        worksheet.write_string_with_format(0, 4, "ID Cliente", &header_format)?;
        worksheet.write_string_with_format(0, 5, "ID Vendedor",&header_format)?;
    
    //rellena los encabezados con los datos de ventas
    for (row, venta) in sales.iter().enumerate() {
        worksheet.write_number((row + 1) as u32, 0, venta.id_venta.unwrap_or(0) as f64)?;
        worksheet.write_string((row + 1) as u32, 1, &venta.matricula)?;
        worksheet.write_string((row + 1) as u32, 2, &venta.fecha_venta.to_string())?;
        worksheet.write_number((row + 1) as u32, 3, venta.precio_venta.to_f64().unwrap_or(0.0))?;
        worksheet.write_number((row + 1) as u32, 4, venta.id_cliente as f64)?;
        worksheet.write_number((row + 1) as u32, 5, venta.id_vendedor as f64)?;        
    }

    workbook.save(file_path)?;
    Ok(())
}
//fin teste