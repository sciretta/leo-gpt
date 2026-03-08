use std::io::Error;

use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MigrationRecord {
    pub name: String,
    pub applied_at: String,
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
}
