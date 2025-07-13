use actix_web::{get, web, App, HttpServer, Responder, Result};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use dotenvy::dotenv;

// Define a struct to hold the database connection pool
#[derive(Debug, Clone)]
struct AppState {
    db: DatabaseConnection,
}

// A simple health check handler
#[get("/health")]
async fn health_check() -> impl Responder {
    "OK"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get the database URL from environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // Establish a connection to the database
    let db_conn = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Successfully connected to the database.");

    let app_state = AppState { db: db_conn };

    println!("Starting server at http://127.0.0.1:8080");

    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(health_check)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
