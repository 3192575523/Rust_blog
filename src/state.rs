use crate::{db::Db, config::Config};

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub cfg: Config,
    pub jwt_secret: String,
}