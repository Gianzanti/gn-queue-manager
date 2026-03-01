use sqlx::PgPool;
use std::ops::Deref;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

impl Deref for AppState {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}
