use actix_files::{Files, NamedFile};
use actix_web::{web, App, HttpServer, Responder, Result};
use serde::Serialize;

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

#[derive(Serialize)]
pub struct Cat {
    pub id: i32,
    pub name: String,
    pub image_path: String,
}

async fn cats_endpoint() -> impl Responder {
    let cats = vec![
        Cat {
            id: 1,
            name: "British Short Hair".to_string(),
            image_path: "image/british-short-hair.jpg".to_string(),
        },
        Cat {
            id: 2,
            name: "Persian".to_string(),
            image_path: "image/persian.jpg".to_string()
        },
        Cat {
            id: 3,
            name: "Ragdoll".to_string(),
            image_path: "image/ragdoll.jpg".to_string(),
        },
    ];
    return web::Json(cats);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on port 8080");
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "static").show_files_listing())
            .service(Files::new("/image", "image").show_files_listing())
            .service(web::scope("/api").route("/cats", web::get().to(cats_endpoint)))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
