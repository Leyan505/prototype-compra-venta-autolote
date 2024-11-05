use actix_files::Files;
use actix_web::{web::Data, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tera::{Context, Tera};

mod services;
use services::fetch_vehicles;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    let query = sqlx::query_file!("bd/seed.sql").execute(&pool).await;

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .app_data(Data::new(tera))
            //.service(create_buy_vehicle)
            .service(Files::new("/static", "./static").show_files_listing()) // Static file handler
            .service(fetch_vehicles)
        //.service(fetch_buys)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
