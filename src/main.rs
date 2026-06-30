use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, routing};
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, query};
use time::OffsetDateTime;
use tokio::{net::TcpListener, task::JoinHandle};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            "cradlog=trace,tower_http=trace,axum::rejection=trace",
        ))
        .with(fmt::layer().pretty())
        .init();

    let pool = PgPool::connect(dotenvy::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();
    let state = Arc::new(AppState { pool });

    let router = Router::new()
        .route("/api/journal", routing::post(create_journal))
        .with_state(state)
        .layer(TraceLayer::new_for_http());
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

#[derive(Serialize, Deserialize)]
struct InferenceMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct InferenceRequest {
    model: String,
    messages: Vec<InferenceMessage>,
}

#[derive(Deserialize)]
struct InferenceResponse {
    choices: Vec<InferenceChoice>,
}

#[derive(Deserialize)]
struct InferenceChoice {
    message: InferenceMessage,
}

#[derive(Debug, Deserialize)]
struct ClinicalEntities {
    diseases: Vec<String>,
    symptoms: Vec<String>,
    suspected_diseases: Vec<String>,
}

async fn create_journal(
    State(state): State<Arc<AppState>>,
    Json(journal): Json<String>,
) -> StatusCode {
    tokio::spawn(async move {
        let result = serde_json::from_str::<ClinicalEntities>(
            Client::new()
                .post(dotenvy::var("LLM_API_URL")?)
                .header(
                    header::AUTHORIZATION,
                    format!("Bearer {}", dotenvy::var("LLM_API_KEY")?,),
                )
                .json(&InferenceRequest {
                    model: dotenvy::var("LLM_API_MODEL")?,
                    messages: vec![
                        InferenceMessage {
                            role: "system".to_owned(),
                            content: include_str!("prompt.txt").to_owned(),
                        },
                        InferenceMessage {
                            role: "user".to_owned(),
                            content: format!("<text>{journal}</text>"),
                        },
                    ],
                })
                .send()
                .await?
                .json::<InferenceResponse>()
                .await?
                .choices[0]
                .message
                .content
                .as_str(),
        )?;

        query!(
            "INSERT INTO diseases VALUES (DEFAULT, $1, $2, $3)",
            if result.diseases.is_empty() {
                &result.suspected_diseases
            } else {
                &result.diseases
            },
            &result.symptoms,
            OffsetDateTime::now_local()?
        )
        .execute(&state.pool)
        .await?;

        Ok(())
    }) as JoinHandle<anyhow::Result<()>>;
    StatusCode::NO_CONTENT
}
