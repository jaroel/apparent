extern crate dotenv;

use actix_files::{Files, NamedFile};
use actix_web::Result as WebResult;
use actix_web::{get, web, App, HttpServer};
use dotenv::dotenv;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

struct Country {
    count: i64,
}

async fn get_country(pool: &PgPool) -> Option<Country> {
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
async fn kekjo(data: web::Data<PgPool>) -> WebResult<web::Json<i32>> {
    // return Ok(web::Json(-1));
    let pool = data.get_ref();
    let result = get_country(pool).await;
    match result {
        Some(value) => Ok(web::Json(value.count as i32)),
        None => Ok(web::Json(-1)),
    }
}

async fn index() -> WebResult<NamedFile> {
    Ok(NamedFile::open("./client/index.html")?)
}

async fn get_pool() -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://postgres:password@localhost/roel")
        .await
        .expect("Can't connect to postgres server.")
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
                    .default_service(web::route().to(web::HttpResponse::NotFound)),
            )
            .service(Files::new("/pkg", "./client/pkg"))
            .default_service(web::get().to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
