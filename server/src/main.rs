pub mod db_collections;
pub mod dtos;
pub mod migrations;
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::sse::{Event, Sse},
    routing::{get, post},
};
use dtos::*;
use futures::stream::{self};
use migrations::*;
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::db_collections::{Chats, Messages, Users};

#[tokio::main]
async fn main() {
    // Connect to MongoDB
    let client = mongodb::Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    let db = Arc::new(client.database("leo_gpt"));

    // Run migrations
    run_migrations(&db).await.unwrap();
    // build our application with a single route
    let app = Router::new()
        .route("/", get(root))
        .route(
            "/chats",
            post(
                |state: State<Arc<mongodb::Database>>, payload: Json<PaginationDTO<UserDTO>>| {
                    get_chats(state, payload)
                },
            ),
        )
        .route(
            "/messages",
            post(
                |state: State<Arc<mongodb::Database>>, payload: Json<PaginationDTO<ChatDTO>>| {
                    get_messages(state, payload)
                },
            ),
        )
        .route(
            "/new_chat",
            post(
                |state: State<Arc<mongodb::Database>>, payload: Json<ChatDTO>| {
                    create_chat(state, payload)
                },
            ),
        )
        .route(
            "/new_message",
            post(
                |state: State<Arc<mongodb::Database>>, payload: Json<MessageDTO>| {
                    create_message_stream(state, payload)
                },
            ),
        )
        .with_state(db.clone());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_chats(
    axum::extract::State(db): axum::extract::State<Arc<mongodb::Database>>,
    Json(payload): Json<PaginationDTO<UserDTO>>,
) -> Result<axum::Json<Vec<Chats>>, String> {
    println!("Received payload: {:?}", payload);
    let user_id = Users::get_id_by_username(&*db, &payload.data.username.unwrap())
        .await
        .map_err(|e| format!("Failed to get user ID: {}", e))?;
    let chats = Chats::get_all_user_chats(&*db, &user_id)
        .await
        .map_err(|e| format!("Failed to get chats: {}", e))?;
    Ok(Json(chats))
}

async fn get_messages(
    axum::extract::State(db): axum::extract::State<Arc<mongodb::Database>>,
    Json(payload): Json<PaginationDTO<ChatDTO>>,
) -> Result<axum::Json<Vec<Messages>>, String> {
    println!("Received payload: {:?}", payload);

    let chat_id: ObjectId = ObjectId::parse_str(&payload.data.chat_id.unwrap())
        .map_err(|e| format!("Invalid chat ID: {}", e))?;

    let messages = Messages::get_all_chat_messages(&*db, chat_id)
        .await
        .map_err(|e| format!("Failed to get messages: {}", e))?;
    Ok(Json(messages))
}

async fn create_chat(
    axum::extract::State(db): axum::extract::State<Arc<mongodb::Database>>,
    Json(payload): Json<ChatDTO>,
) -> Result<axum::Json<Chats>, String> {
    println!("Received payload: {:?}", payload);

    let chat_name = payload.chat_name.expect("param chat_name must be provided");
    let user_id: ObjectId = ObjectId::parse_str(&payload.user_id.unwrap())
        .map_err(|e| format!("Invalid user ID: {}", e))?;

    let messages = Chats::create_chat(&*db, &user_id, &chat_name)
        .await
        .map_err(|e| format!("Failed to get messages: {}", e))?;
    Ok(Json(messages))
}

async fn create_message_stream(
    axum::extract::State(db): axum::extract::State<Arc<mongodb::Database>>,
    Json(payload): Json<MessageDTO>,
) -> Result<
    Sse<impl futures::stream::Stream<Item = Result<Event, axum::Error>>>,
    (StatusCode, String),
> {
    println!("Received payload: {:?}", payload);

    let chat_id: ObjectId = ObjectId::parse_str(&payload.chat_id.unwrap())
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid chat ID: {}", e)))?;

    let body = payload.body.expect("param body must be provided");

    let (tx, rx) = mpsc::channel(100);

    tokio::spawn(async move {
        Messages::create_message_stream(&*db, &chat_id, body, |chunk| {
            let tx_clone = tx.clone();
            let chunk_clone = chunk.clone();
            tokio::spawn(async move {
                tx_clone.send(chunk_clone).await.unwrap();
            });
        })
        .await
        .unwrap();
    });

    let stream = stream::unfold(rx, |mut rx| async move {
        match rx.recv().await {
            Some(chunk) => {
                let event = Event::default().data(chunk);
                Some((Ok(event), rx))
            }
            None => None,
        }
    });

    Ok(Sse::new(stream))
}
