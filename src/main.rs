use axum::{Json, Router, http::StatusCode, routing};
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, task::JoinHandle};
use tower_http::trace::TraceLayer;
use tracing::{debug, info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            "cradlog=trace,tower_http=trace,axum::rejection=trace",
        ))
        .with(fmt::layer().pretty())
        .init();
    let router = Router::new()
        .route("/api/journal", routing::post(create_journal))
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
#[allow(unused)]
struct ClinicalEntities {
    diseases: Vec<String>,
    symptoms: Vec<String>,
    suspected_diseases: Vec<String>,
}

async fn create_journal(Json(journal): Json<String>) -> StatusCode {
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
        debug!(?result);
        Ok(())
    }) as JoinHandle<anyhow::Result<()>>;
    StatusCode::NO_CONTENT
}
