use cradlog::{route, types::Context};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            "cradlog=trace,tower_http=trace,axum::rejection=trace",
        ))
        .with(fmt::layer().pretty())
        .init();

    let router = route::init(Context {
        db: PgPool::connect(&dotenvy::var("DATABASE_URL").expect("env `DATABASE_URL` not found"))
            .await
            .expect("failed to connect to the database"),
    });
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

// #[derive(Serialize, Deserialize)]
// struct InferenceMessage {
//     role: String,
//     content: String,
// }

// #[derive(Serialize)]
// struct InferenceRequest {
//     model: String,
//     messages: Vec<InferenceMessage>,
// }

// #[derive(Deserialize)]
// struct InferenceResponse {
//     choices: Vec<InferenceChoice>,
// }

// #[derive(Deserialize)]
// struct InferenceChoice {
//     message: InferenceMessage,
// }

// #[derive(Debug, Deserialize)]
// struct ClinicalEntities {
//     diseases: Vec<String>,
//     symptoms: Vec<String>,
//     suspected_diseases: Vec<String>,
// }

// #[derive(Serialize)]
// struct Disease {
//     id: i64,
//     diseases: Vec<String>,
//     symptoms: Vec<String>,

//     #[serde(with = "time::serde::rfc3339")]
//     creation_time: OffsetDateTime,
// }

// #[derive(Deserialize)]
// struct JournalUpdateParams {
//     diseases: Option<Vec<String>>,
//     symptoms: Option<Vec<String>>,
// }

// async fn create_journal(
//     State(state): State<Arc<AppState>>,
//     Json(journal): Json<String>,
// ) -> StatusCode {
//     tokio::spawn(async move {
//         let result = serde_json::from_str::<ClinicalEntities>(
//             Client::new()
//                 .post(dotenvy::var("LLM_API_URL")?)
//                 .header(
//                     header::AUTHORIZATION,
//                     format!("Bearer {}", dotenvy::var("LLM_API_KEY")?,),
//                 )
//                 .json(&InferenceRequest {
//                     model: dotenvy::var("LLM_API_MODEL")?,
//                     messages: vec![
//                         InferenceMessage {
//                             role: "system".to_owned(),
//                             content: include_str!("prompt.txt").to_owned(),
//                         },
//                         InferenceMessage {
//                             role: "user".to_owned(),
//                             content: format!("<text>{journal}</text>"),
//                         },
//                     ],
//                 })
//                 .send()
//                 .await?
//                 .json::<InferenceResponse>()
//                 .await?
//                 .choices[0]
//                 .message
//                 .content
//                 .as_str(),
//         )?;

//         query!(
//             "INSERT INTO diseases VALUES (DEFAULT, $1, $2, $3)",
//             if result.diseases.is_empty() {
//                 &result.suspected_diseases
//             } else {
//                 &result.diseases
//             },
//             &result.symptoms,
//             OffsetDateTime::now_local()?,
//         )
//         .execute(&state.pool)
//         .await?;

//         Ok(())
//     }) as JoinHandle<anyhow::Result<()>>;
//     StatusCode::NO_CONTENT
// }

// async fn query_journals(
//     State(state): State<Arc<AppState>>,
// ) -> Result<Json<Vec<Disease>>, StatusCode> {
//     match query_as!(
//         Disease,
//         "SELECT * FROM diseases ORDER BY creation_time DESC",
//     )
//     .fetch_all(&state.pool)
//     .await
//     {
//         Ok(diseases) => Ok(Json(diseases)),
//         Err(e) => {
//             error!(?e);
//             Err(StatusCode::INTERNAL_SERVER_ERROR)
//         }
//     }
// }

// async fn update_journal(
//     State(state): State<Arc<AppState>>,
//     Path(id): Path<i64>,
//     Json(params): Json<JournalUpdateParams>,
// ) -> StatusCode {
//     match query!(
//         "UPDATE diseases SET diseases = COALESCE($1, diseases), symptoms = COALESCE($2, symptoms) WHERE id = $3",
//         params.diseases.as_deref(),
//         params.symptoms.as_deref(),
//         id,
//     )
//     .execute(&state.pool)
//     .await {
//         Ok(result) if result.rows_affected() > 0 => StatusCode::NO_CONTENT,
//         Ok(_) => StatusCode::NOT_FOUND,
//         Err(e) => {
//             error!(?e);
//             StatusCode::INTERNAL_SERVER_ERROR
//         }
//     }
// }

// async fn delete_journal(State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> StatusCode {
//     match query!("DELETE FROM diseases WHERE id = $1", id)
//         .execute(&state.pool)
//         .await
//     {
//         Ok(result) if result.rows_affected() > 0 => StatusCode::NO_CONTENT,
//         Ok(_) => StatusCode::NOT_FOUND,
//         Err(e) => {
//             error!(?e);
//             StatusCode::INTERNAL_SERVER_ERROR
//         }
//     }
// }
