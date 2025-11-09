use axum::{
    Router,
    routing::{get, post},
    // routing::{put, delete},
};
use tower_http::services::ServeDir;
use crate::state::AppState;

pub mod public;
pub mod admin;
pub mod me;

pub fn app_router(state: AppState) -> Router {
    let upload_dir = state.cfg.upload_dir.clone();

    Router::new()
        // ===== public =====
        .route("/health", get(public::health))
        .route("/api/posts", get(public::list_posts))
        .route("/api/posts/slug/:slug", get(public::get_post)) // 公开详情（仅已发布+public；作者登录可看自己的 private，逻辑在 public::get_post 内实现）
        .route("/api/tags", get(public::list_tags))
        .route("/rss.xml", get(public::rss))
        .route("/sitemap.xml", get(public::sitemap))

        // ===== admin（需登录）=====
        .route("/api/auth/login", post(admin::login))
        .route("/api/posts", post(admin::create_post))
        // ✅ 新增 GET：作者本人按 id 读取（编辑页用）；保留 PUT/DELETE
        .route(
            "/api/posts/:id",
            get(admin::get_post).put(admin::update_post).delete(admin::delete_post)
        )
        .route("/api/posts/:id/publish", post(admin::publish_post))
        .route("/api/media", post(admin::upload_media))

        // me（作者自服务） 👇
        .route("/api/me", get(me::get_me).put(me::update_me))
        .route("/api/me/posts", get(me::list_my_posts))

        // 静态文件（开发期本地看上传）
        .nest_service("/uploads", ServeDir::new(upload_dir))

        // 共享全局状态（一次）
        .with_state(state)
}
