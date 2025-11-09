use std::net::SocketAddr;
use tracing_subscriber::{EnvFilter, fmt::Subscriber};

mod auth;
mod config;
mod db;
mod error;
mod markdown;
mod models;
mod routes;
mod rss;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1) 读取 .env（如果存在）
    dotenvy::dotenv().ok();

    // 2) 日志
    Subscriber::builder()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("sqlx=warn".parse()?)
                .add_directive("axum::rejection=warn".parse()?),
        )
        .init();

    // 3) 配置 & 数据库
    let cfg = config::Config::new()?;
    let pool = db::init_pool(&cfg.database_url).await?;
    // 如果你有内置迁移，这里会执行
    db::migrate(&pool).await?;

    // 4) （可选但推荐）最佳努力在线迁移：为 posts 添加 visibility 列与索引
    // 已存在时执行会报错，这里用 let _ = 忽略即可；没有表/列时则添加成功
    let _ = sqlx::query(
        "ALTER TABLE posts ADD COLUMN visibility TEXT \
         CHECK(visibility IN ('public','private')) DEFAULT 'public';",
    )
    .execute(&pool)
    .await;

    let _ = sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_posts_status_visibility_pubat \
         ON posts(status, visibility, published_at DESC);",
    )
    .execute(&pool)
    .await;

    // 5) 应用状态
    let app_state = state::AppState {
        db: pool.clone(),
        cfg: cfg.clone(),
        jwt_secret: cfg.jwt_secret.clone(),
    };

    // 6) 构建路由（routes::app_router 内部已 .with_state(app_state)）
    let app = routes::app_router(app_state);

    // 7) 绑定 & 启动 —— ★★★ 保持你要求的三行完全不变 ★★★
    let addr: SocketAddr = cfg.bind.parse()?;
    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    Ok(())
}

async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        if let Err(_e) = signal::ctrl_c().await {
            // 忽略错误
        }
    };

    #[cfg(unix)]
    let terminate = async {
        if let Ok(mut sig) = signal::unix::signal(signal::unix::SignalKind::terminate()) {
            sig.recv().await;
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
