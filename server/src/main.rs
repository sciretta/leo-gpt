use axum::{Router, routing::get};
pub mod db_collections;
pub mod migrations;
use migrations::*;

#[tokio::main]
async fn main() {
    // Connect to MongoDB
    let client = mongodb::Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    let db = client.database("leo_gpt");

    // Run migrations
    run_migrations(&db).await.unwrap();
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
