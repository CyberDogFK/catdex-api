mod models;
mod schema;

use self::models::*;
use self::schema::cats::dsl::*;
use actix_files::{Files, NamedFile};
use actix_web::{error, web, App, Error, HttpResponse, HttpServer, Result};
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use std::env;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

async fn cats_endpoint(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().expect("Can't get db connection from pool");
    let cats_data = web::block(move || cats.limit(100).load::<Cat>(&mut connection))
        .await
        .map_err(error::ErrorInternalServerError)?
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(cats_data))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.");

    println!("Listening on port 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(Files::new("/static", "static").show_files_listing())
            .service(Files::new("/image", "image").show_files_listing())
            .service(web::scope("/api").route("/cats", web::get().to(cats_endpoint)))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
