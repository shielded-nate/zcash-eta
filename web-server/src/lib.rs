pub mod cli;

use std::net::SocketAddr;

use axum::{
    Json, Router,
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
};
use include_dir::{Dir, include_dir};
use serde::{Deserialize, Serialize};
use zcash_eta_offline::{
    CurrentHeightDifficultyTimestamp, height_intervals_for_timestamp,
    timestamp_intervals_for_height,
};

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

#[derive(Debug, Deserialize)]
pub struct ConvertQuery {
    pub current_height: u64,
    pub current_difficulty: f64,
    pub current_timestamp: i64,
    pub target_height: Option<u64>,
    pub target_timestamp: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ConvertResponse {
    TimestampIntervals(serde_json::Value),
    HeightIntervals(serde_json::Value),
}

pub fn app() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/convert", get(convert))
        .route("/static/{*path}", get(static_file))
}

pub async fn run_server(addr: SocketAddr) -> color_eyre::Result<()> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app()).await?;
    Ok(())
}

async fn index() -> impl IntoResponse {
    match STATIC_DIR.get_file("index.html") {
        Some(file) => Html(file.contents_utf8().unwrap_or_default().to_owned()).into_response(),
        None => (StatusCode::NOT_FOUND, "missing index.html").into_response(),
    }
}

async fn static_file(axum::extract::Path(path): axum::extract::Path<String>) -> impl IntoResponse {
    match STATIC_DIR.get_file(path.as_str()) {
        Some(file) => (
            [("content-type", "text/plain; charset=utf-8")],
            file.contents().to_vec(),
        )
            .into_response(),
        None => (StatusCode::NOT_FOUND, "not found").into_response(),
    }
}

pub async fn convert(Query(query): Query<ConvertQuery>) -> impl IntoResponse {
    let current = CurrentHeightDifficultyTimestamp {
        height: query.current_height,
        difficulty: query.current_difficulty,
        timestamp: query.current_timestamp,
    };

    match (query.target_height, query.target_timestamp) {
        (Some(target_height), None) => Json(ConvertResponse::TimestampIntervals(
            serde_json::to_value(timestamp_intervals_for_height(current, target_height)).unwrap(),
        ))
        .into_response(),
        (None, Some(target_timestamp)) => Json(ConvertResponse::HeightIntervals(
            serde_json::to_value(height_intervals_for_timestamp(current, target_timestamp))
                .unwrap(),
        ))
        .into_response(),
        _ => (
            StatusCode::BAD_REQUEST,
            "provide exactly one of target_height or target_timestamp",
        )
            .into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn convert_height_query_returns_json() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/convert?current_height=1000&current_difficulty=1.5&current_timestamp=1700000000&target_height=1010")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn index_is_embedded() {
        let response = app()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
