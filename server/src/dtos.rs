use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationDTO<T> {
    pub page: u32,
    pub per_page: u32,
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDTO {
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatDTO {
    pub chat_id: Option<String>,
    pub chat_name: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDTO {
    pub chat_id: Option<String>,
    pub body: Option<String>,
}
