use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing,
};
use tower_http::trace::TraceLayer;
use tracing::error;

use crate::{
    svc,
    types::{Context, LogInParams},
};

pub fn init(ctx: Context) -> Router {
    Router::new()
        .route("/api/log", routing::post(create_log))
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(ctx))
}

pub async fn create_log(
    State(ctx): State<Arc<Context>>,
    Json(params): Json<LogInParams>,
) -> Result<StatusCode, Error> {
    svc::log::create(&ctx, params).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug)]
pub struct Error(anyhow::Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("unexpected error: {:?}", self.0);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

impl<E: Into<anyhow::Error>> From<E> for Error {
    fn from(value: E) -> Self {
        Self(value.into())
    }
}
