use serde::Deserialize;
// ! use serde::Serialize;
use uuid::Uuid;
use chrono::Utc;
// ! use sqlx::FromRow;

// ! #[derive(Debug, Serialize, FromRow)]
// ! pub struct Post {
// !     pub id: String,
// !     pub slug: String,
// !     pub title: String,
// !     pub excerpt: Option<String>,
// !     pub body_md: String,
// !     pub body_html: String,
// !     pub status: String,
// !     pub author_id: String,
// !     pub published_at: Option<String>,
// !     pub created_at: String,
// !     pub updated_at: String,
// ! }

#[derive(Debug, Deserialize)]
pub struct PostInput {
    pub title: String,
    pub slug: Option<String>,
    pub body_md: String,
    pub tags: Option<Vec<String>>,  // tag 名称数组
    pub status: Option<String>,     // draft/published
    pub excerpt: Option<String>,
    // ✅ 新增：public/private
    pub visibility: Option<String>,
}

pub fn new_id() -> String { Uuid::new_v4().to_string() }
pub fn now() -> String { Utc::now().to_rfc3339() }
