use crate::{
    auth::{self, AuthUser},
    error::AppError,
    markdown,
    models::{new_id, now, PostInput},
    state::AppState,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::{Multipart, Path, State},
    Json,
};
use regex::Regex;
use serde::Deserialize;
use sqlx::{self, Transaction};
use std::{fs, path::PathBuf};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

/// POST /api/auth/login
pub async fn login(
    State(app): State<AppState>,
    Json(inp): Json<LoginInput>,
) -> Result<Json<serde_json::Value>, AppError> {
    let row = sqlx::query!(
        r#"SELECT
            id            as "id!: String",
            password_hash as "password_hash!: String"
          FROM users WHERE username = ?"#,
        inp.username
    )
    .fetch_optional(&app.db)
    .await?;

    let r = row.ok_or(AppError::Unauthorized)?;
    let parsed = PasswordHash::new(&r.password_hash).map_err(|_| AppError::Unauthorized)?;
    Argon2::default()
        .verify_password(inp.password.as_bytes(), &parsed)
        .map_err(|_| AppError::Unauthorized)?;

    let token = auth::sign(&r.id, app.jwt_secret.as_str(), 60 * 24)?; // 24h
    Ok(Json(serde_json::json!({ "access_token": token })))
}

/// POST /api/posts
pub async fn create_post(
    State(app): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Json(inp): Json<PostInput>,
) -> Result<Json<serde_json::Value>, AppError> {
    let id = new_id();
    let slug = inp.slug.clone().unwrap_or_else(|| slugify(&inp.title));
    let html = markdown::render(&inp.body_md);
    let now_ts = now();
    let status = inp.status.clone().unwrap_or_else(|| "draft".into());
    let visibility = inp
        .visibility
        .clone()
        .unwrap_or_else(|| "public".into());

    let mut tx = app.db.begin().await?;
    sqlx::query!(
        r#"INSERT INTO posts (
            id, slug, title, excerpt, body_md, body_html, status, visibility, author_id, created_at, updated_at
          ) VALUES (?,?,?,?,?,?,?,?,?,?,?)"#,
        id,
        slug,
        inp.title,
        inp.excerpt,
        inp.body_md,
        html,
        status,
        visibility,
        user_id,
        now_ts,
        now_ts
    )
    .execute(&mut *tx)
    .await?;

    if let Some(tags) = &inp.tags {
        for name in tags {
            let tag_id = ensure_tag(&mut tx, name).await?;
            sqlx::query!(
                "INSERT OR IGNORE INTO post_tags (post_id, tag_id) VALUES (?,?)",
                id,
                tag_id
            )
            .execute(&mut *tx)
            .await?;
        }
    }

    tx.commit().await?;
    Ok(Json(serde_json::json!({ "id": id, "slug": slug })))
}

/// PUT /api/posts/:id
pub async fn update_post(
    State(app): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Path(id): Path<String>,
    Json(inp): Json<PostInput>,
) -> Result<Json<serde_json::Value>, AppError> {
    let slug = inp.slug.clone().unwrap_or_else(|| slugify(&inp.title));
    let html = markdown::render(&inp.body_md);
    let ts = now();
    let status = inp.status.clone().unwrap_or_else(|| "draft".into());
    let visibility = inp
        .visibility
        .clone()
        .unwrap_or_else(|| "public".into());

    // 只允许作者本人更新
    let res = sqlx::query!(
        r#"
        UPDATE posts SET
            slug        = ?,
            title       = ?,
            excerpt     = ?,
            body_md     = ?,
            body_html   = ?,
            status      = ?,
            visibility  = ?,
            updated_at  = ?
        WHERE id = ? AND author_id = ?
        "#,
        slug,
        inp.title,
        inp.excerpt,
        inp.body_md,
        html,
        status,
        visibility,
        ts,
        id,
        user_id
    )
    .execute(&app.db)
    .await?;

    if res.rows_affected() == 0 {
        return Err(AppError::Forbidden);
    }

    // 若传了 tags，则整体重建关联（简单粗暴，但清晰可靠）
    if let Some(tags) = &inp.tags {
        let mut tx = app.db.begin().await?;
        sqlx::query!("DELETE FROM post_tags WHERE post_id = ?", id)
            .execute(&mut *tx)
            .await?;
        for name in tags {
            let tag_id = ensure_tag(&mut tx, name).await?;
            sqlx::query!(
                "INSERT OR IGNORE INTO post_tags (post_id, tag_id) VALUES (?,?)",
                id,
                tag_id
            )
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
    }

    Ok(Json(serde_json::json!({ "ok": true })))
}

/// POST /api/posts/:id/publish
pub async fn publish_post(
    State(app): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let ts = now();
    let res = sqlx::query!(
        "UPDATE posts SET status='published', published_at=?, updated_at=? WHERE id=? AND author_id=?",
        ts,
        ts,
        id,
        user_id
    )
    .execute(&app.db)
    .await?;

    if res.rows_affected() == 0 {
        return Err(AppError::Forbidden);
    }
    Ok(Json(serde_json::json!({ "ok": true })))
}

/// DELETE /api/posts/:id
pub async fn delete_post(
    State(app): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let res = sqlx::query!("DELETE FROM posts WHERE id=? AND author_id=?", id, user_id)
        .execute(&app.db)
        .await?;
    if res.rows_affected() == 0 {
        return Err(AppError::Forbidden);
    }
    Ok(Json(serde_json::json!({ "ok": true })))
}

/// POST /api/media
pub async fn upload_media(
    State(app): State<AppState>,
    AuthUser { .. }: AuthUser,
    mut mp: Multipart,
) -> Result<Json<serde_json::Value>, AppError> {
    fs::create_dir_all(&app.cfg.upload_dir).ok();
    let mut saved = vec![];

    while let Some(field) = mp
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?
    {
        let fname = field
            .file_name()
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("f-{}", Uuid::new_v4()));
        let data = field
            .bytes()
            .await
            .map_err(|e| AppError::BadRequest(e.to_string()))?;

        let mut path = PathBuf::from(&app.cfg.upload_dir);
        path.push(&fname);
        tokio::fs::write(&path, &data)
            .await
            .map_err(|e| AppError::BadRequest(e.to_string()))?;

        let url = format!("/uploads/{}", fname); // 生产环境建议交给 Nginx 静态托管
        saved.push(url);
    }

    Ok(Json(serde_json::json!({ "files": saved })))
}

fn slugify(title: &str) -> String {
    let s = title.trim().to_lowercase();
    let re = Regex::new(r"[^a-z0-9]+").unwrap();
    let s = re.replace_all(&s, "-").to_string();
    s.trim_matches('-').to_string()
}

async fn ensure_tag(
    tx: &mut Transaction<'_, sqlx::Sqlite>,
    name: &str,
) -> Result<String, AppError> {
    let slug = slugify(name);
    let row = sqlx::query!(
        r#"SELECT id as "id!: String" FROM tags WHERE slug = ?"#,
        slug
    )
    .fetch_optional(&mut **tx)
    .await?;

    if let Some(row) = row {
        return Ok(row.id);
    }

    let id = Uuid::new_v4().to_string();
    sqlx::query!(
        "INSERT INTO tags (id, slug, name) VALUES (?,?,?)",
        id,
        slug,
        name
    )
    .execute(&mut **tx)
    .await?;

    Ok(id)
}

// GET /api/posts/:id  —— 仅作者本人可读取（用于编辑页加载原文）
pub async fn get_post(
    State(app): State<AppState>,
    AuthUser { user_id }: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    // 主体内容（确保非空列都带上 "col!: Type" 断言）
    let row = sqlx::query!(
        r#"
        SELECT
            id            as "id!: String",
            slug          as "slug!: String",
            title         as "title!: String",
            excerpt,
            body_md       as "body_md!: String",
            body_html     as "body_html!: String",
            status        as "status!: String",
            visibility    as "visibility!: String",
            published_at,
            author_id     as "author_id!: String"
        FROM posts
        WHERE id = ? AND author_id = ?
        "#,
        id,
        user_id
    )
    .fetch_optional(&app.db)
    .await?;

    // 不是作者或不存在 → 拒绝
    let p = row.ok_or(AppError::Forbidden)?;

    // 标签列表
    let tag_rows = sqlx::query!(
        r#"
        SELECT t.name
          FROM tags t
          JOIN post_tags pt ON pt.tag_id = t.id
         WHERE pt.post_id = ?
        "#,
        p.id
    )
    .fetch_all(&app.db)
    .await?;

    let tags: Vec<String> = tag_rows.into_iter().map(|r| r.name).collect();

    Ok(Json(serde_json::json!({
        "id": p.id,
        "slug": p.slug,
        "title": p.title,
        "excerpt": p.excerpt,
        "body_md": p.body_md,
        "body_html": p.body_html,
        "status": p.status,
        "visibility": p.visibility,
        "published_at": p.published_at,
        "author_id": p.author_id,
        "tags": tags
    })))
}
