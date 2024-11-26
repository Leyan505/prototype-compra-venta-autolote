
use actix_web::{get,post,web, HttpResponse, Responder};
use serde::Deserialize;
use tera::Tera;
use crate::{Context, AppState};

#[derive(Deserialize)]
pub struct LoginData {
    username: String,
    password: String,
}


#[get("/login")]
pub async fn login_page(tera: web::Data<Tera>) -> impl Responder {
    let context = Context::new();
    match tera.render("log/login.html", &context) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(_) => HttpResponse::InternalServerError().body("Error rendering login papito"),
    }
}


#[post("/login")]
pub async fn login_user(
    state: web::Data<AppState>,
    form: web::Form<LoginData>,
) -> impl Responder {
    match sqlx::query!(
        "SELECT username FROM users WHERE username = $1 AND password_hash = crypt($2, password_hash)",
        form.username,  
        form.password
    )
    .fetch_optional(&state.db)
    .await
    {
        Ok(Some(_user)) => {
            HttpResponse::Found()
            .append_header(("Location", "/"))
            .finish()
        },
        Ok(_) => HttpResponse::Unauthorized().body("Invalid username or password"),
        Err(_) => HttpResponse::InternalServerError().body("Error validating user"),
    }
}