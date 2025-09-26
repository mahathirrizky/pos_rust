mod entities;
mod migration; // Declare the migration module
mod handler;
mod repository;
mod helper;
mod routes;
mod auth;
mod middleware;
mod extractor;
mod guard; // New module
mod websocket; // <-- MODUL BARU

use std::env;
use actix::Actor;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait; // Import MigratorTrait
use actix_cors::Cors;
use actix_web::{http, web, App, HttpRequest, HttpServer, Result};
use actix_files::{Files, NamedFile};
use env_logger;

use crate::websocket::broadcaster::Broadcaster;
use crate::websocket::start_ws_connection;

use crate::auth::auth_handler::login;
use crate::middleware::role::RoleMiddlewareFactory;

async fn index(_req: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("./frontend/dist/index.html")?)
}

#[actix_web::main] // Make main function async and actix compatible
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db: DatabaseConnection = Database::connect(&database_url).await.expect("Failed to connect to database");

    migration::Migrator::up(&db, None).await.expect("Failed to run migrations");

    // Mulai Broadcaster Actor
    let broadcaster = Broadcaster::default().start();

    let db_data = web::Data::new(db);
    let broadcaster_data = web::Data::new(broadcaster);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(db_data.clone())
            .app_data(broadcaster_data.clone()) // <-- Tambahkan state broadcaster
            .service(web::scope("/api/auth").route("/login", web::post().to(login)))
            .service(
                web::scope("/api")
                    .wrap(RoleMiddlewareFactory { allowed_roles: vec![] }) // Membutuhkan autentikasi tetapi tidak membatasi peran
                    .configure(routes::configure_routes)
            )
            
            // WebSocket route
            .service(start_ws_connection) // <-- Tambahkan rute WebSocket

            // Static assets
            .service(Files::new("/assets", "./frontend/dist/assets"))
            .route("/vite.svg", web::get().to(|| async { NamedFile::open_async("./frontend/dist/vite.svg").await }))

            // SPA routes (should be last)
            .route("/{tail:.*}", web::get().to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
