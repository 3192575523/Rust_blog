// src/routes/me.rs
use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{QueryBuilder, Row, Sqlite};
use crate::{auth::AuthUser, error::AppError, state::AppState};

#[derive(Serialize)]
pub struct MeResp {
    pub id: String,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub motto: Option<String>,
    pub created_at: String,
}

/// GET /api/me  （需要登录）
pub async fn get_me(
    State(app): State<AppState>,
    AuthUser { user_id }: AuthUser,
) -> Result<Json<MeResp>, AppError> {
    let r = sqlx::query!(
        r#"
        SELECT
          id           as "id!: String",
          username     as "username!: String",
          display_name as "display_name: String",
          avatar_url   as "avatar_url: String",
          motto        as "motto: String",
          created_at   as "created_at!: String"
        FROM users WHERE id = ?
        "#,
        user_id
    )
    .fetch_one(&app.db)
    .await?;

    Ok(Json(MeResp {
        id: r.id,
        username: r.username,
        display_name: r.display_name,
        avatar_url: r.avatar_url,
        motto: r.motto,
        created_at: r.created_at,
    }))
}

#[derive(Deserialize, Debug)]
pub struct MePatch {
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub motto: Option<String>,
}

/// PUT /api/me  （需要登录）
/// 仅更新传入的字段；未传入的保持不变
pub async fn update_me(
    State(app): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Json(patch): Json<MePatch>,
) -> Result<Json<serde_json::Value>, AppError> {
    // 用 COALESCE(?, column) 保持未提供的字段不变（Option -> NULL）
    sqlx::query!(
        r#"
        UPDATE users SET
          display_name = COALESCE(?, display_name),
          avatar_url   = COALESCE(?, avatar_url),
          motto        = COALESCE(?, motto)
        WHERE id = ?
        "#,
        patch.display_name,
        patch.avatar_url,
        patch.motto,
        user_id
    )
    .execute(&app.db)
    .await?;

    Ok(Json(serde_json::json!({ "ok": true })))
}

#[derive(Deserialize, Debug)]
pub struct MyPostsParams {
    pub status: Option<String>,        // all|published|draft
    pub visibility: Option<String>,    // all|public|private
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

/// GET /api/me/posts  （需要登录）
/// 返回当前作者的文章列表，支持 status / visibility 筛选与分页
pub async fn list_my_posts(
    State(app): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Query(p): Query<MyPostsParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = p.page.unwrap_or(1).max(1);
    let size = p.page_size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * size;

    let mut qb = QueryBuilder::<Sqlite>::new(
        "SELECT id, slug, title, excerpt, status, visibility, published_at
           FROM posts
          WHERE author_id = ",
    );
    qb.push_bind(&user_id);

    // status 过滤
    if let Some(s) = p.status.as_deref() {
        match s {
            "published" => { qb.push(" AND status = 'published'"); }
            "draft"     => { qb.push(" AND status = 'draft'"); }
            _ => {} // all 或非法值：不加过滤
        }
    }

    // visibility 过滤
    if let Some(v) = p.visibility.as_deref() {
        match v {
            "public"  => { qb.push(" AND visibility = 'public'"); }
            "private" => { qb.push(" AND visibility = 'private'"); }
            _ => {} // all 或非法值：不加过滤
        }
    }

    qb.push(" ORDER BY COALESCE(published_at, updated_at) DESC, updated_at DESC");
    qb.push(" LIMIT ").push_bind(size).push(" OFFSET ").push_bind(offset);

    let rows = qb.build().fetch_all(&app.db).await?;

    let items: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "id":           r.get::<String,_>("id"),
                "slug":         r.get::<String,_>("slug"),
                "title":        r.get::<String,_>("title"),
                "excerpt":      r.try_get::<String,_>("excerpt").ok(),
                "status":       r.get::<String,_>("status"),
                "visibility":   r.get::<String,_>("visibility"),
                "published_at": r.try_get::<String,_>("published_at").ok(),
            })
        })
        .collect();

    Ok(Json(serde_json::json!({
        "page": page,
        "page_size": size,
        "items": items
    })))
}
