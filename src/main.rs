use actix_web::{web::Data, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sales::get_sales;
use sellers::{obtain_sellers, post_sellers};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use statics::read_static;
use tera::{Context, Tera};

mod sellers;
mod sales;
mod statics;
mod vehicles;
use vehicles::{delete_vehicles, edit_vehicles, fetch_vehicles, get_vehicle, insert_vehicles};

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

    let _query = sqlx::query_file!("bd/seed.sql").execute(&pool).await;

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .app_data(Data::new(tera))
            //.service(create_buy_vehicle)
            .service(read_static) // Static file handler
            .service(fetch_vehicles)
            .service(post_sellers)
            .service(obtain_sellers)
            .service(get_sales)
            // .service(in_sales)
            .service(insert_vehicles)
            .service(get_vehicle)
            .service(delete_vehicles)
            .service(edit_vehicles)
        //.service(fetch_buys)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
