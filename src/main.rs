mod database;
mod handlers;
mod logging;
mod models;
mod settings;
mod validation;

use actix_web::{middleware::Logger as ActixLogger, web, App, HttpServer};
use anyhow::Result;
use std::sync::Arc;

use crate::database::Database;
use crate::handlers::{handle_delete, handle_get, handle_patch, handle_post, AppState};
use crate::logging::Logger;
use crate::settings::Settings;

#[actix_web::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Load settings
    let settings = Settings::load()?;
    println!("Settings loaded: {:?}", settings);

    // Initialize database
    let db = Arc::new(Database::new(&settings)?);
    println!("Database initialized");

    // Initialize logger
    let logger = Arc::new(Logger::new()?);
    println!("Logger initialized");

    // Create app state
    let app_state = web::Data::new(AppState { db, logger });

    let bind_address = format!("{}:{}", settings.host, settings.port);
    println!("Starting GResources server on {}", bind_address);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(ActixLogger::default())
            .route("/{path:.*}", web::post().to(handle_post))
            .route("/{path:.*}", web::get().to(handle_get))
            .route("/{path:.*}", web::patch().to(handle_patch))
            .route("/{path:.*}", web::delete().to(handle_delete))
            .route("/", web::get().to(handle_get))
            .route("/", web::post().to(handle_post))
            .route("/", web::patch().to(handle_patch))
            .route("/", web::delete().to(handle_delete))
    })
    .bind(bind_address)?
    .run()
    .await?;

    Ok(())
}
