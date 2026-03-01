use crate::app_state::AppState;
use axum::{Router, routing::get};
use controllers::*;
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

mod app_state;
mod controllers;
mod models;
mod visitor;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!("src/migrations")
        .run(&pool)
        .await
        .expect("Couldn't migrate the database tables");

    let state = AppState::new(pool);

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/visitors", get(retrieve_all_visitor).post(create_visitor))
        .route(
            "/visitors/{id}",
            get(retrieve_visitor_by_id)
                .put(update_visitor_by_id)
                .delete(delete_visitor_by_id),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_private_network(true)
                .allow_headers(Any),
        )
        .with_state(state);

    Ok(router.into())
}
