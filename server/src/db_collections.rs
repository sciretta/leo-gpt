use std::{
    io::Error,
    sync::{Arc, Mutex},
};

use futures::StreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use ollama_rs::{
    Ollama,
    generation::chat::{ChatMessage, ChatMessageResponse, request::ChatMessageRequest},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MigrationRecord {
    pub name: String,
    pub applied_at: String,
}

pub trait UserBasics {
    async fn user_exists(db: &mongodb::Database, user_id: &ObjectId) -> Result<bool, Error> {
        let collection = db.collection::<Users>("users");
        let filter = doc! { "_id": user_id };
        Ok(collection.find_one(filter).await.unwrap().is_some())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Users {
    #[serde(default, rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub username: String,
    pub created_at: String,
}

impl Users {
    pub async fn get_id_by_username(
        db: &mongodb::Database,
        username: &str,
    ) -> Result<ObjectId, Error> {
        let collection = db.collection::<Users>("users");
        let filter = doc! { "username": username };
        if let Some(user) = collection.find_one(filter).await.unwrap() {
            Ok(user._id.unwrap())
        } else {
            Err(Error::new(std::io::ErrorKind::NotFound, "User not found"))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chats {
    #[serde(default, rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub chat_name: String,
    pub created_at: String,
}

impl Chats {
    pub async fn get_all_user_chats(
        db: &mongodb::Database,
        user_id: &ObjectId,
    ) -> Result<Vec<Chats>, Error> {
        let collection = db.collection::<Chats>("chats");
        let filter = doc! { "user_id": user_id };
        let mut cursor = collection.find(filter).await.unwrap();
        let mut chats = Vec::new();
        while cursor.advance().await.unwrap() {
            chats.push(cursor.deserialize_current().unwrap());
        }
        Ok(chats)
    }

    pub async fn create_chat(
        db: &mongodb::Database,
        user_id: &ObjectId,
        chat_name: &str,
    ) -> Result<Chats, Error> {
        let user_exists = Self::user_exists(db, user_id).await.unwrap();
        if !user_exists {
            return Err(Error::new(std::io::ErrorKind::NotFound, "User not found"));
        }
        let collection = db.collection::<Chats>("chats");
        let new_chat = Chats {
            _id: None,
            user_id: user_id.clone(),
            chat_name: chat_name.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        let insert_result = collection.insert_one(new_chat.clone()).await.unwrap();
        Ok(Chats {
            _id: Some(insert_result.inserted_id.as_object_id().unwrap()),
            ..new_chat
        })
    }
}

impl UserBasics for Chats {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Messages {
    #[serde(default, rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub chat_id: ObjectId,
    pub is_user: bool,
    pub content: String,
    pub created_at: String,
}

impl Messages {
    pub async fn get_all_chat_messages(
        db: &mongodb::Database,
        chat_id: ObjectId,
    ) -> Result<Vec<Messages>, Error> {
        let collection = db.collection::<Messages>("messages");
        let filter = doc! { "chat_id": chat_id };
        let mut cursor = collection.find(filter).await.unwrap();
        let mut messages = Vec::new();
        while cursor.advance().await.unwrap() {
            messages.push(cursor.deserialize_current().unwrap());
        }
        Ok(messages)
    }

    // pub async fn create_message(
    //     db: &mongodb::Database,
    //     chat_id: &ObjectId,
    //     body: String,
    // ) -> Result<Messages, Error> {
    //     let model = "llama3".to_string();

    //     let ollama = Ollama::default();

    //     let mut history: Vec<ChatMessage> = Vec::new();

    //     let prev_messages = Self::get_all_chat_messages(db, chat_id.clone())
    //         .await
    //         .unwrap();

    //     for msg in prev_messages {
    //         if msg.is_user {
    //             history.push(ChatMessage::user(msg.content));
    //         } else {
    //             history.push(ChatMessage::assistant(msg.content));
    //         }
    //     }

    //     let request = ChatMessageRequest::new(model.clone(), vec![ChatMessage::user(body.clone())]);

    //     let resp = ollama
    //         .send_chat_messages_with_history(&mut history, request)
    //         .await
    //         .unwrap();

    //     let assistant_text = resp.message.content.clone();

    //     let collection = db.collection::<Messages>("messages");
    //     let new_message = Messages {
    //         _id: None,
    //         chat_id: chat_id.clone(),
    //         is_user: true,
    //         content: body,
    //         created_at: chrono::Utc::now().to_rfc3339(),
    //     };
    //     let insert_result = collection.insert_one(new_message.clone()).await.unwrap();
    //     insert_result.inserted_id.as_object_id().unwrap();

    //     let assistant_message = Messages {
    //         _id: None,
    //         chat_id: chat_id.clone(),
    //         is_user: false,
    //         content: assistant_text,
    //         created_at: chrono::Utc::now().to_rfc3339(),
    //     };
    //     let insert_result = collection
    //         .insert_one(assistant_message.clone())
    //         .await
    //         .unwrap();
    //     insert_result.inserted_id.as_object_id().unwrap();

    //     Ok(assistant_message)
    // }

    pub async fn create_message_stream<F>(
        db: &mongodb::Database,
        chat_id: &ObjectId,
        body: String,
        mut on_chunk: F,
    ) -> Result<(), Error>
    where
        F: FnMut(String),
    {
        let model = "llama3".to_string();
        let ollama = Ollama::default();
        let mut history: Vec<ChatMessage> = Vec::new();

        let prev_messages = Self::get_all_chat_messages(db, chat_id.clone())
            .await
            .unwrap();

        for msg in prev_messages {
            if msg.is_user {
                history.push(ChatMessage::user(msg.content));
            } else {
                history.push(ChatMessage::assistant(msg.content));
            }
        }

        let request = ChatMessageRequest::new(model.clone(), vec![ChatMessage::user(body.clone())]);

        println!("History: {:?}", history);

        let history = Arc::new(Mutex::new(history));

        let mut resp = ollama
            .send_chat_messages_with_history_stream(history, request)
            .await
            .unwrap();

        let mut assistant_text = String::new();

        while let Some(chunk_result) = resp.next().await {
            match chunk_result {
                Ok(chunk) => {
                    let chunk_content = chunk.message.content;
                    assistant_text.push_str(&chunk_content);
                    on_chunk(chunk_content); // Send chunk to callback
                }
                Err(e) => {
                    eprintln!("Error receiving chunk: {:?}", e);
                    break;
                }
            }
        }

        // Save user message
        let collection = db.collection::<Messages>("messages");
        let new_message = Messages {
            _id: None,
            chat_id: chat_id.clone(),
            is_user: true,
            content: body,
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        let insert_result = collection.insert_one(new_message.clone()).await.unwrap();
        insert_result.inserted_id.as_object_id().unwrap();

        // Save assistant message
        let assistant_message = Messages {
            _id: None,
            chat_id: chat_id.clone(),
            is_user: false,
            content: assistant_text,
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        let insert_result = collection
            .insert_one(assistant_message.clone())
            .await
            .unwrap();
        insert_result.inserted_id.as_object_id().unwrap();

        Ok(())
    }
}

impl UserBasics for Messages {}
