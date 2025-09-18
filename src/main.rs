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

use std::env;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait; // Import MigratorTrait
use actix_web::{web, App, HttpServer};

#[actix_web::main] // Make main function async and actix compatible
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db: DatabaseConnection = Database::connect(&database_url).await.expect("Failed to connect to database");

    migration::Migrator::up(&db, None).await.expect("Failed to run migrations");

    let db_data = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            // Public routes (e.g., login)
            .service(web::scope("/api/auth").route("/login", web::post().to(auth::auth_handler::login)))
            // Protected routes are now configured directly in routes
            .service(
                web::scope("/api")
                    .configure(routes::configure_routes)
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
