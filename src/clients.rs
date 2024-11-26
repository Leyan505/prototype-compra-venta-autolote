use crate::AppState;
use actix_web::{
    error::InternalError,
    get,
    http::{header::FROM, StatusCode},
    post,
    web::{self, Json, Redirect},
    HttpResponse, Responder, ResponseError,
};
use rust_xlsxwriter::{Color, Format, Workbook, XlsxError};
use serde::{de::Error, Deserialize, Serialize};
use sqlx::{
    self,
    error::DatabaseError,
    types::{chrono, BigDecimal},
    FromRow,
};
use std::{fmt, option};
use std::string;
use tera::{Context, Tera};

#[derive(Serialize, Deserialize, FromRow)]
struct Cliente {
    id_cliente: Option<i32>,
    nombre: String,
    apellido: String,
    cedula: String,
}
impl fmt::Display for Cliente {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.nombre, self.apellido)
    }
}
#[get("/clients")]
pub async fn fetch_clients(state: web::Data<AppState>, tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();

    match sqlx::query_as!(
        Cliente,
        r#"SELECT id_cliente, nombre, apellido, cedula FROM cliente
        WHERE estado != 'OUT'"#
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(clientes) => {
            context.insert("clientes", &clientes);
            HttpResponse::Ok().body(tera.render("clients.html", &context).unwrap())
        }
        Err(_) => HttpResponse::NotFound().json("No clients found"),
    }
}

#[post("/clients")]
pub async fn insert_client(
    state: web::Data<AppState>,
    new_client: web::Form<Cliente>,
) -> impl Responder {
    let mut context = Context::new();

    match sqlx::query_as!(
        Cliente,
        r#"
        INSERT INTO cliente (nombre, apellido, cedula)
        VALUES ($1, $2, $3)
    "#,
        new_client.nombre,
        new_client.apellido,
        new_client.cedula,
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => HttpResponse::SeeOther()
            .append_header(("Location", "/clients"))
            .finish(),
        Err(_) => HttpResponse::InternalServerError().body("Error al insertar vehiculo"),
    }
}

#[post("/delete_client/{id}")]
pub async fn delete_client(
    state: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> impl Responder {
    let id = path.into_inner().0;
    let result = sqlx::query!(
        r#"
        UPDATE cliente
        SET estado = 'OUT'
        WHERE id_cliente = $1
    "#,
        id,
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => HttpResponse::SeeOther()
            .append_header(("Location", "/clients"))
            .finish(),
        Err(_) => {
            HttpResponse::InternalServerError().body("Error al eliminar la venta")
        }
    }
}

#[get("/clients/clientDetails/{id}")]
pub async fn get_client(
    state: web::Data<AppState>,
    path: web::Path<(String,)>,
) -> Result<impl Responder, InternalError<String>> {
    let id: i32 = path
        .into_inner()
        .0
        .parse()
        .expect("failed to parse string to integer");
    match sqlx::query_as!(
        Cliente,
        r#"
    SELECT id_cliente, nombre, apellido, cedula
    FROM cliente
    WHERE id_cliente = $1
    "#,
        id
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(client) => Ok(web::Json(client)),
        Err(err) => Err(InternalError::new(
            err.to_string(), // Convert the error to a string or handle it accordingly
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        )), // Error case
    }
}

#[post("/edit_client/")]
pub async fn edit_client(
    state: web::Data<AppState>,
    modified_client: web::Form<Cliente>,
) -> Result<HttpResponse, InternalError<String>> {
    match sqlx::query!(
        r#"
    UPDATE cliente
    SET nombre = $1, apellido = $2, cedula = $3
    WHERE id_cliente = $4
    "#,
        modified_client.nombre,
        modified_client.apellido,
        modified_client.cedula,
        modified_client.id_cliente
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/clients"))
            .finish()),
        Err(err) => Err(InternalError::new(
            err.to_string(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

// test export to xls
#[get("/clients/export")]
pub async fn export_clients(state: web::Data<AppState>) -> impl Responder {
    let vehiculos_result = sqlx::query_as!(
        Cliente,
        r#"SELECT id_cliente, nombre, apellido, cedula FROM cliente
        WHERE estado != 'OUT'"#
    )
    .fetch_all(&state.db)
    .await;

    match vehiculos_result {
        Ok(vehiculos) => {
            match export_clients_to_xlsx(vehiculos, "export/archivo.xlsx").await {
                Ok(_) => {
                    match std::fs::read("export/archivo.xlsx") {
                        Ok(file_bytes) => {
                            HttpResponse::Ok()
                                .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
                                .insert_header(("Content-Disposition", "attachment; filename=clients.xlsx"))
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
pub async fn export_clients_to_xlsx(clients: Vec<Cliente>, file_path: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let header_format = Format::new()
    .set_font_size(12.0)
    .set_bold()
    .set_background_color(Color::Green);

 

    // esrcibe los encabezados
    worksheet.write_string_with_format(0, 0, "id_cliente", &header_format)?;
    worksheet.write_string_with_format(0, 1, "nombre", &header_format)?;
    worksheet.write_string_with_format(0, 2, "apellido", &header_format)?;
    worksheet.write_string_with_format(0, 3, "cedula", &header_format)?;

    
    //rellena los encabezados con los datos de ventas
    for (row, cliente) in clients.iter().enumerate() {
        worksheet.write_number((row + 1) as u32, 0, cliente.id_cliente.unwrap())?;
        worksheet.write_string((row + 1) as u32, 1, &cliente.nombre)?;
        worksheet.write_string((row + 1) as u32, 2, &cliente.apellido)?;
        worksheet.write_string((row + 1) as u32, 3, &cliente.cedula)?;     

    }

    workbook.save(file_path)?;
    Ok(())
}
//fin teste