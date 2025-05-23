// src/main.rs

// mod hari1_pengantar_rust {
//     pub mod day1_basic;
// }

// mod hari2_control_flow {
//     pub mod day2_control;
// }

// mod hari3_ownership {
//     pub mod day3_ownership;
// }

// mod hari4_struct_enum {
//     pub mod day4_struct_enum;
// }

// mod hari5_collections_error {
//     pub mod day5_collections_error;
// }

// fn main() {
//     println!("Mulai Program Rust...");
//     hari5_collections_error::day5_collections_error::run();
// }

mod backend;

use warp::Filter;
use sqlx::MySqlPool;
use dotenv::dotenv;

use backend::models;
use backend::database;
use backend::utils;
use backend::auth::handlers;

use handlers::{register_handler, login_handler};
use database::create_connection_pool;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Create database connection pool
    let pool = create_connection_pool()
        .await
        .expect("Failed to create database pool");

    // CORS configuration
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type", "authorization"])
        .allow_methods(vec!["POST", "GET", "PUT", "DELETE", "OPTIONS"]);

    // Routes
    let register_route = warp::path("api")
        .and(warp::path("register"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and_then(register_handler);

    let login_route = warp::path("api")
        .and(warp::path("login"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and_then(login_handler);

    let health_route = warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::json(&serde_json::json!({
            "status": "OK",
            "message": "Auth API is running"
        })));

    let routes = register_route
        .or(login_route)
        .or(health_route)
        .with(cors);

    println!("Auth API server running on http://localhost:8080");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

fn with_db(pool: MySqlPool) -> impl Filter<Extract = (MySqlPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
