extern crate dotenv;
mod ws;

use std::env;

use actix_files::{Files, NamedFile};
use actix_web::Result as WebResult;
use actix_web::{get, web, App, HttpServer};
use dotenv::dotenv;

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

use crate::ws::ws_index;

struct Country {
    count: i32,
}

async fn get_country(pool: &SqlitePool) -> Option<Country> {
    let result = sqlx::query_as!(Country, r#"SELECT COUNT(*) as "count!" FROM countries"#)
        .fetch_one(pool)
        .await;
    match result {
        Ok(value) => Some(value),
        Err(_error) => {
            println!("{}", _error);
            return None;
        }
    }
}

#[get("kekjo")]
async fn kekjo(data: web::Data<SqlitePool>) -> WebResult<web::Json<i32>> {
    // return Ok(web::Json(-1));
    let pool = data.get_ref();
    let result = get_country(pool).await;
    match result {
        Some(value) => Ok(web::Json(value.count)),
        None => Ok(web::Json(-1)),
    }
}

#[get("non_db")]
async fn non_db() -> WebResult<web::Json<i32>> {
    return Ok(web::Json(1));
}

async fn index() -> WebResult<NamedFile> {
    Ok(NamedFile::open("./client/index.html")?)
}

async fn get_pool() -> SqlitePool {
    let database_url = env::var("DATABASE_URL").expect("missing DATABASE_URL");
    SqlitePoolOptions::new()
        .min_connections(2)
        .max_connections(16)
        .test_before_acquire(false)
        .connect(&database_url)
        .await
        .expect("No DB")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = get_pool().await;

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(
                web::scope("/api/")
                    .service(kekjo)
                    .service(non_db)
                    .default_service(web::route().to(web::HttpResponse::NotFound)),
            )
            .service(ws_index)
            .service(Files::new("/pkg", "./client/pkg"))
            .default_service(web::get().to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
