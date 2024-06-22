use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct ListResponse {
    pub id: Uuid,
    pub title: String,
    pub descr: Option<String>,
    pub body: Option<String>,
    pub user_id: Uuid,
    pub importance: String, // high, medium , low
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateListSchema {
    pub title: String,
    pub descr: Option<String>,
    pub body: Option<String>,
    pub importance: String, // high, medium , low
}

#[derive(Deserialize)]
pub struct PaginationSchema {
    pub page: Option<usize>,
    pub page_size: Option<usize>,
    pub search_title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateListSchema {
    pub title: Option<String>,
    pub descr: Option<String>,
    pub body: Option<String>,
    pub importance: Option<String>, // high, medium , low
    pub id: Uuid,
}
