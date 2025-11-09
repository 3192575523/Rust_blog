// ! 处理信号连接和读取配置
#[derive(Clone)]
pub struct Config {
    pub bind: String,  //"0.0.0.0:8080"
    pub database_url: String, // "sqlite://blog.db"
    pub jwt_secret: String, // "jwt secret key"
    pub upload_dir: String, // "uploads"
    pub site_base: String, // "http://example.com"
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
                bind: std::env::var("BIND").unwrap_or("0.0.0.0:8080".into()),
                database_url: std::env::var("DATABASE_URL")?,
                jwt_secret: std::env::var("JWT_SECRET")?,
                upload_dir: std::env::var("UPLOAD_DIR").unwrap_or("./uploads".into()),
                site_base: std::env::var("SITE_BASE").unwrap_or("http://localhost:8080".into()),
        })
    }
}