use axum::http::Method;
use axum::{routing::get, Router};
use controllers::*;
use sqlx::migrate::MigrateDatabase;
use sqlx::{Pool, Sqlite, SqlitePool};
use tower_http::cors::{AllowOrigin, CorsLayer};

mod controllers;
mod visitor;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

const DB_URL: &str = "sqlite://gn.sqlite3";

#[derive(Clone)]
pub struct AppState {
    db: Pool<Sqlite>,
}

impl AppState {
    fn new(db: Pool<Sqlite>) -> Self {
        Self { db }
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        Sqlite::create_database(DB_URL).await.unwrap();
        // match create_schema(&DB_URL).await {
        //     Ok(_) => println!("Database created Sucessfully"),
        //     Err(e) => panic!("{}", e),
        // }
    }
    let pool = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::migrate!("src/migrations")
        .run(&pool)
        .await
        .expect("Couldn't migrate the database tables");

    let state = AppState::new(pool);

    let router = Router::new()
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::any())
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS]),
        )
        .route("/", get(hello_world))
        .route("/visitors", get(retrieve_all_visitor).post(create_visitor))
        .route(
            "/visitors/:id",
            get(retrieve_visitor_by_id)
                .put(update_visitor_by_id)
                .delete(delete_visitor_by_id),
        )
        .with_state(state);

    Ok(router.into())
}
