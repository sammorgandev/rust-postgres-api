use std::sync::Arc;

use axum::{
    //axum is the http server framework
    body::{Body, to_bytes},
    Extension,
    extract::{Json, Path, Request},
    http::StatusCode,
    response::Response,
};
use chrono::Utc;
use serde_json::{from_slice, json, Value};
use tokio_postgres::Client;

use crate::models::Post;

//CUSTOM HANDLERS
pub async fn get_all_posts(client: Arc<Client>) -> Result<Json<Value>, Response> {
    let result = Post::get_all(client).await;
    match result {
        Ok(posts) => Ok(Json(json!({ "posts": posts }))),
        Err(e) => {
            let error_message = format!("Failed to fetch posts: {}", e);
            let error_response = json!({"error": error_message});
            let body = Body::from(serde_json::to_string(&error_response).unwrap());
            Err(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(body)
                .unwrap())
        }
    }
}

pub async fn get_posts_by_category(
    client: Extension<Arc<Client>>,
    category: String,
) -> Result<Json<Value>, Response> {
    let result = Post::get_category(client, category).await;
    match result {
        Ok(posts) => Ok(Json(json!({ "posts": posts }))),
        Err(e) => {
            let error_message = format!("Failed to fetch posts: {}", e);
            let error_response = json!({"error": error_message});
            let body = Body::from(serde_json::to_string(&error_response).unwrap());
            Err(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(body)
                .unwrap())
        }
    }
}

pub async fn get_posts_by_tag(
    client: Extension<Arc<Client>>,
    tag: String,
) -> Result<Json<Value>, Response> {
    let result = Post::get_tag(client, tag).await;
    match result {
        Ok(posts) => Ok(Json(json!({ "posts": posts }))),
        Err(e) => {
            let error_message = format!("Failed to fetch posts: {}", e);
            let error_response = json!({"error": error_message});
            let body = Body::from(serde_json::to_string(&error_response).unwrap());
            Err(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(body)
                .unwrap())
        }
    }
}

pub async fn get_post(
    Path(post_slug): Path<String>,
    Extension(client): Extension<Arc<Client>>,
) -> Result<Json<Post>, StatusCode> {
    match Post::get(client, post_slug).await {
        Ok(Some(post)) => Ok(Json(post)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn add_post(
    client: Arc<Client>,
    req: Request<Body>,
) -> Result<Json<Value>, axum::response::Response> {
    let body_bytes = to_bytes(req.into_body(), usize::MAX).await.unwrap();
    let post: Post = from_slice(&body_bytes).unwrap();

    match Post::new(post, client).await {
        Ok(_) => {
            let success_message = Json(json!({"message": "Post added successfully"}));
            Ok(success_message)
        }
        Err(e) => {
            let error_message = format!("Failed to decode token: {:?}", e);
            let error_response = json!({"error": error_message});
            let body = Body::from(serde_json::to_string(&error_response).unwrap());
            Err(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(body)
                .unwrap())
        }
    }
}

pub async fn delete_post(
    req: Request<Body>,
    Extension(client): Extension<Arc<Client>>,
) -> Result<Json<Value>, axum::response::Response> {
    let body_bytes = to_bytes(req.into_body(), usize::MAX).await.unwrap();
    let post: Post = from_slice(&body_bytes).unwrap();
    let post_id = post.id;

    match Post::delete(client, post_id.into()).await {
        Ok(_) => {
            let success_message = Json(json!({"message": "Post deleted successfully"}));
            Ok(success_message)
        }
        Err(e) => {
            let error_message = format!("Failed to delete post: {}", e);
            let error_response = json!({"error": error_message});
            let body = Body::from(serde_json::to_string(&error_response).unwrap());
            Err(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(body)
                .unwrap())
        }
    }
}

pub async fn update_post(
    req: Request<Body>,
    Extension(client): Extension<Arc<Client>>,
) -> Result<Json<Value>, Response> {
    let body_bytes = to_bytes(req.into_body(), usize::MAX).await.unwrap();
    let post: Post = from_slice(&body_bytes).unwrap();
    let post_id = post.id;

    match Post::update(client, post).await {
        Ok(_) => {
            let success_message = Json(json!({"message": "Post updated successfully"}));
            Ok(success_message)
        }
        Err(e) => {
            let error_message = format!("Failed to update post: {}", e);
            let error_response = json!({"error": error_message});
            let body = Body::from(serde_json::to_string(&error_response).unwrap());
            Err(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(body)
                .unwrap())
        }
    }
}
