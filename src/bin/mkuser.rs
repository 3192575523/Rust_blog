// src/bin/mkuser.rs
use anyhow::Result;
use argon2::{Argon2, password_hash::{SaltString, PasswordHasher}};
use rand::rngs::OsRng;
use sqlx::sqlite::SqlitePoolOptions;
use chrono::Utc;
use uuid::Uuid;
use std::env;

#[tokio::main]
// 这是一个异步主函数，用于创建新用户
// 返回一个 Result 类型，表示可能的错误
// 这是一个异步函数，用于创建新用户
// 返回 Result 类型，用于处理可能的错误
async fn main() -> Result<()> {
    // 从 .env 文件加载环境变量，如果出错则忽略
    dotenvy::dotenv().ok();
    // 收集命令行参数到字符串向量中
    let args: Vec<String> = env::args().collect();
    // 检查参数数量是否足够（至少需要用户名和密码）
    if args.len() < 3 {
        // 打印错误信息和使用说明
        eprintln!("用法: cargo run --bin mkuser -- <username> <password>");
        // 以非零状态码退出程序
        std::process::exit(1);
    }
    // 从参数中提取用户名和密码
    let username = &args[1];
    let password = &args[2];

    // 从环境变量中获取数据库URL
    let db_url = env::var("DATABASE_URL")?;
    // 创建SQLite连接池
    let pool = SqlitePoolOptions::new().connect(&db_url).await?;

    // 生成随机盐值
    let salt = SaltString::generate(&mut OsRng);
    // 使用Argon2算法对密码进行哈希处理
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    // 获取当前时间并转换为RFC3339格式
    let now = Utc::now().to_rfc3339();
    // 生成一个新的UUID作为用户ID
    let id = Uuid::new_v4().to_string();

    // 执行SQL插入语句，将新用户信息存入数据库
    sqlx::query!("INSERT INTO users (id, username, password_hash, created_at) VALUES (?,?,?,?)",
        id, username, hash, now
    ).execute(&pool).await?;

    // 打印成功创建用户的信息
    println!("OK: 已创建用户 `{}`", username);
    // 返回成功结果
    Ok(())
}
