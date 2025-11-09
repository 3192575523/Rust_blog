use axum::{
    extract::{FromRequestParts, Path, Query, State},
    http::request::Parts,
    Json,
};
use serde::Deserialize;
use sqlx::{self, FromRow, QueryBuilder, Row, Sqlite};
use crate::{auth::AuthUser, error::AppError, rss as rss_mod, state::AppState};

#[derive(Deserialize)]
pub struct ListParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub tag: Option<String>,
    pub q: Option<String>,
}

pub async fn health() -> &'static str {
    "ok"
}

/// 允许“可选登录”的提取器：带 token 则解析为 Some(AuthUser)，否则 None
pub struct MaybeUser(pub Option<AuthUser>);

#[axum::async_trait]
impl<S> FromRequestParts<S> for MaybeUser
where
    AuthUser: FromRequestParts<S, Rejection = AppError>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match AuthUser::from_request_parts(parts, state).await {
            Ok(u) => Ok(MaybeUser(Some(u))),
            Err(_) => Ok(MaybeUser(None)),
        }
    }
}

/// GET /api/posts
/// 仅返回「已发布 + public」的文章。支持分页、tag 与关键字搜索。
pub async fn list_posts(
    State(app): State<AppState>,
    Query(p): Query<ListParams>,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = p.page.unwrap_or(1).max(1);
    let size = p.page_size.unwrap_or(10).clamp(1, 50);
    let offset = (page - 1) * size;

    // 构造一条查询
    let mut qb = QueryBuilder::<Sqlite>::new(
        "SELECT id, slug, title, excerpt, published_at \
         FROM posts WHERE status='published' AND visibility='public'",
    );

    // 一次性初始化，避免“赋值后未读取”的告警
    let tag_val: Option<String> = p.tag.clone();
    let like_val: Option<String> = p.q.as_ref().map(|s| format!("%{}%", s));

    if let Some(ref tag_s) = tag_val {
        qb.push(" AND id IN (\
            SELECT post_id FROM post_tags \
             WHERE tag_id IN (SELECT id FROM tags WHERE slug = ")
            .push_bind(tag_s)
            .push(" OR name = ")
            .push_bind(tag_s)
            .push("))");
    }

    if let Some(ref like_s) = like_val {
        qb.push(" AND (title LIKE ")
            .push_bind(like_s)
            .push(" OR body_html LIKE ")
            .push_bind(like_s)
            .push(")");
    }

    qb.push(" ORDER BY published_at DESC LIMIT ")
        .push_bind(size)
        .push(" OFFSET ")
        .push_bind(offset);

    let rows = qb.build().fetch_all(&app.db).await?;

    let items: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "id": r.get::<String,_>("id"),
                "slug": r.get::<String,_>("slug"),
                "title": r.get::<String,_>("title"),
                "excerpt": r.try_get::<String,_>("excerpt").ok(),
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

#[derive(FromRow)]
struct PostRow {
    id: String,
    slug: String,
    title: String,
    excerpt: Option<String>,
    body_html: String,
    published_at: Option<String>,
    author_id: String,
    visibility: String,
    status: String,
}

/// GET /api/posts/slug/:slug
/// 匿名/非作者：只看 public；作者登录后：可看自己 private。
pub async fn get_post(
    State(app): State<AppState>,
    MaybeUser(mu): MaybeUser,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut qb = QueryBuilder::<Sqlite>::new(
        "SELECT id, slug, title, excerpt, body_html, published_at, author_id, visibility, status \
         FROM posts WHERE slug = ",
    );
    qb.push_bind(&slug);
    qb.push(" AND status = 'published'");

    if let Some(AuthUser { user_id }) = mu {
        qb.push(" AND (visibility='public' OR author_id = ")
            .push_bind(user_id)
            .push(")");
    } else {
        qb.push(" AND visibility='public'");
    }

    let rec: Option<PostRow> = qb
        .build_query_as::<PostRow>()
        .fetch_optional(&app.db)
        .await?;

    let p = rec.ok_or(AppError::NotFound)?;
    Ok(Json(serde_json::json!({
        "id": p.id,
        "slug": p.slug,
        "title": p.title,
        "excerpt": p.excerpt,
        "body_html": p.body_html,
        "published_at": p.published_at,
        "author_id": p.author_id,
        "visibility": p.visibility,
        "status": p.status
    })))
}

/// GET /api/tags
pub async fn list_tags(State(app): State<AppState>) -> Result<Json<Vec<serde_json::Value>>, AppError> {
    let rows = sqlx::query!(
        r#"
        SELECT t.id, t.slug, t.name, COUNT(pt.post_id) as cnt
          FROM tags t LEFT JOIN post_tags pt ON t.id = pt.tag_id
         GROUP BY t.id
         ORDER BY cnt DESC
        "#
    )
    .fetch_all(&app.db)
    .await?;

    Ok(Json(
        rows.into_iter()
            .map(|r| {
                serde_json::json!({
                    "id": r.id,
                    "slug": r.slug,
                    "name": r.name,
                    "count": r.cnt
                })
            })
            .collect(),
    ))
}

/// GET /rss.xml
pub async fn rss(
    State(app): State<AppState>,
) -> Result<(axum::http::HeaderMap, String), AppError> {
    let xml = rss_mod::build_rss(&app.db, &app.cfg).await?;
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "application/rss+xml".parse().unwrap(),
    );
    Ok((headers, xml))
}

/// GET /sitemap.xml
pub async fn sitemap(
    State(app): State<AppState>,
) -> Result<(axum::http::HeaderMap, String), AppError> {
    let xml = rss_mod::build_sitemap(&app.db, &app.cfg).await?;
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "application/xml".parse().unwrap(),
    );
    Ok((headers, xml))
}
