use crate::{
    visitor::{Visitor, VisitorSubmission, VisitorUpdateRecord},
    AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub async fn retrieve_all_visitor(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let res = match sqlx::query_as::<_, Visitor>("SELECT * FROM VISITORS")
        .fetch_all(&state.db)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {e}"),
            ));
        }
    };

    Ok(Json(res))
}

pub async fn retrieve_visitor_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let res = match sqlx::query_as::<_, Visitor>("SELECT * FROM VISITORS WHERE ID = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {e}"),
            ));
        }
    };

    Ok(Json(res))
}

pub async fn create_visitor(
    State(state): State<AppState>,
    Json(json): Json<VisitorSubmission>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query(
        "INSERT INTO VISITORS (name, customer, phone, email, lgpd, image_rights) VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(json.name)
    .bind(json.customer)
    .bind(json.phone)
    .bind(json.email)
    .bind(json.lgpd)
    .bind(json.image_rights)
    .execute(&state.db)
    .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {e}"),
        ));
    }
    Ok(StatusCode::CREATED)
}

pub async fn update_visitor_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(json): Json<VisitorUpdateRecord>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query(
        "UPDATE VISITORS 
                SET
                 name = (case when $1 is not null then $1 else name end),
                 phone = (case when $2 is not null then $2 else phone end),
                 email = (case when $3 is not null then $3 else email end),
                 lgpd = (case when $4 is not null then $4 else lgpd end),
                 image_rights = (case when $5 is not null then $5 else image_rights end),
                 customer = (case when $6 is not null then $6 else customer end),
                 observations = (case when $7 is not null then $7 else observations end),
                 confirm_visit = (case when $8 is not null then $8 else confirm_visit end),
                 updated_at = NOW()
                WHERE id = $9",
    )
    .bind(json.name)
    .bind(json.phone)
    .bind(json.email)
    .bind(json.lgpd)
    .bind(json.image_rights)
    .bind(json.customer)
    .bind(json.observations)
    .bind(json.confirm_visit)
    .bind(id)
    .execute(&state.db)
    .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}

pub async fn delete_visitor_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query("DELETE FROM VISITORS WHERE ID = $1")
        .bind(id)
        .execute(&state.db)
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {e}"),
        ));
    }

    Ok(StatusCode::OK)
}
