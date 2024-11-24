use actix_web::{web::Data, App, HttpRequest, HttpResponse, HttpServer, Responder};
use costs::{delete_costs, fetch_costs, get_cost, insert_costs};
use dotenv::dotenv;
use dashboard::{index};
use sales::{delete_sales, edit_sales, export_sales, get_sales, get_sales_details, insert_sales};
use sellers::{delete_sellers, edit_sellers, get_seller_details, get_sellers, insert_sellers};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use statics::read_static;
use tera::{Context, Tera};

mod sellers;
mod dashboard;
mod sales;
mod statics;
mod costs;
mod vehicles;
mod clients;
use clients::{fetch_clients, delete_client, edit_client, insert_client, get_client};
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
            //sellers
            .service(insert_sellers)
            .service(get_sellers)
            .service(get_seller_details)
            .service(delete_sellers)
            .service(edit_sellers)
            //sales
            .service(get_sales)
            .service(get_sales_details)
            .service(insert_sales)
            .service(delete_sales)
            .service(edit_sales)
            .service(export_sales)
            //end sales
            .service(insert_vehicles)
            .service(get_vehicle)
            .service(delete_vehicles)
            .service(edit_vehicles)
            .service(get_cost)
            .service(fetch_costs)
            .service(delete_costs)
            .service(insert_costs)

            //dashboard
            .service(index)

            //clients
            .service(fetch_clients)
            .service(delete_client)
            .service(edit_client)
            .service(insert_client)
            .service(get_client)         
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
