// 链接数据库
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

pub type Db = Pool<Sqlite>;

pub async fn init_pool(url: &str) -> anyhow::Result<Db> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(url).await?;
    Ok(pool)
}

pub async fn migrate(pool: &Db) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}