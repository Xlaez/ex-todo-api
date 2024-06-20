use crate::{
    models::{ListModel, UserModel},
    schemas::{CreateListSchema, PaginationSchema},
    AppState,
};
use axum::{
    extract::{Extension, Json, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_macros::debug_handler;
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[debug_handler]
pub async fn add_list_handler(
    State(data): State<Arc<AppState>>,
    Extension(current_user): Extension<UserModel>,
    Json(body): Json<CreateListSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let list = get_list_by_title(&body.title, &current_user.id, &data.db).await;

    match list {
        Some(_) => {
            let error_response =
                json!({"status": "fail", "message": "You already have an item with this title"});
            Err((StatusCode::NOT_ACCEPTABLE, Json(error_response)))
        }
        None => {
            let descr = body.descr.unwrap_or_default();
            let body_content = body.body.unwrap_or_default();
            let importance = body.importance.to_string();

            let query_result = sqlx::query!(
                "INSERT INTO lists (user_id, title, descr, body, importance) VALUES ($1, $2, $3, $4, $5) RETURNING id, user_id, title, descr, body, importance, created_at, updated_at",
                current_user.id,
                body.title,
                descr,
                body_content,
                importance
            )
            .fetch_one(&data.db)
            .await;

            match query_result {
                Ok(row) => {
                    let list = ListModel {
                        id: row.id,
                        user_id: row.user_id.unwrap(),
                        title: row.title,
                        descr: row.descr,
                        body: row.body,
                        importance: row.importance.unwrap(),
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                    };
                    let list_response = json!({"status": "success", "data": {"list": list}});
                    Ok(Json(list_response))
                }
                Err(err) => {
                    let error_response =
                        json!({"status": "fail", "message": format!("Cannot add item: {:?}", err)});
                    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
                }
            }
        }
    }
}

pub async fn get_users_lists_handler(
    State(data): State<Arc<AppState>>,
    Extension(current_user): Extension<UserModel>,
    Query(pagination): Query<PaginationSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let page = pagination.page.unwrap_or(1);
    let page_size = (pagination.page_size.unwrap_or(10));
    let offset = ((page - 1) * page_size) as i64;

    let search_title = pagination.search_title.unwrap_or_default();
    let search_pattern = format!("%{}%", search_title);

    let total_count = sqlx::query!(
        "SELECT COUNT(*) FROM lists WHERE user_id = $1 AND title ILIKE $2",
        current_user.id,
        search_pattern
    )
    .fetch_one(&data.db)
    .await
    .map(|row| row.count)
    .unwrap_or(Some(0));

    let has_more = total_count > Some(offset + page_size as i64);
    let next_page = if has_more { Some(page + 1) } else { None };
    let prev_page = if page > 1 { Some(page - 1) } else { None };

    match sqlx::query!(
        "SELECT * FROM lists WHERE user_id = $1 AND title ILIKE $2 LIMIT $3 OFFSET $4",
        current_user.id,
        search_pattern,
        page_size as i64,
        offset
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(rows) => {
            let lists: Vec<ListModel> = rows
                .into_iter()
                .map(|row| ListModel {
                    id: row.id,
                    title: row.title,
                    user_id: row.user_id.unwrap(),
                    descr: row.descr,
                    body: row.body,
                    importance: row.importance.unwrap(),
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                })
                .collect();

            let response = serde_json::json!({
            "status": "success",
            "data":
            {
                "lists": lists,
                "hasMore": has_more,
                "nextPage": next_page,
                "prevPage": prev_page,
                "totalCount": total_count
            }});

            Ok(Json(response))
        }
        Err(err) => {
            let error_response =
                json!({"status": "fail", "message": format!("Cannot fetch lists: {:?}", err)});
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

pub async fn get_list_by_title<'a>(
    title: &'a str,
    user_id: &'a Uuid,
    pool: &PgPool,
) -> Option<ListModel> {
    match sqlx::query!(
        "SELECT * FROM lists WHERE title = $1 AND user_id = $2",
        title,
        user_id
    )
    .fetch_one(pool)
    .await
    {
        Ok(row) => Some(ListModel {
            id: row.id,
            title: row.title,
            user_id: row.user_id?,
            descr: row.descr,
            body: row.body,
            importance: row.importance?,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }),
        Err(_) => None,
    }
}
