use actix_web::{web, App, HttpServer, http};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use sqlx::PgPool;

pub mod routes;
pub mod configuration;

use crate::routes::{health_check, add, remove};
use configuration::*;

#[actix_web::main]
async fn main() {
    // Start the logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Load config
    let config = get_config("../config.yaml".to_string())
        .expect("Failed to load settings.");

    // Connect to database
    let pool = PgPool::connect(database::connection_string(config.database).as_str())
        .await
        .expect("Failed to connect to Postgres database.");
    let shared_pool = web::Data::new(pool);
    
    let _server = HttpServer::new(move || {
    	// Configure CORS
	    let cors = Cors::default()
            .allowed_origin("http://127.0.0.1")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
        	.max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(shared_pool.clone())
            .route("/healthcheck", web::get().to(health_check::health_check))
            .route("/add", web::post().to(add::add))
            .route("/remove", web::post().to(remove::remove))
            .route("/show", web::get().to(add::show))
        })
        .bind(server::connect(config.server))
        .unwrap()
        .run()
        .await;
}