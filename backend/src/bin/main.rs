use tower_http::trace::TraceLayer;
use whatsapp_analyzer::{analyze_zip_bytes, models::{errors::MyError, result::WhatsResult}};
use axum::{
    routing::post,
    Router,
    extract::Multipart,
    Json
};
use tracing_subscriber;
use tracing_subscriber::prelude::*;
use tower_http::cors::{CorsLayer, Any};
use tracing::{info};

async fn process_upload(mut multipart: Multipart) -> Result<Json<WhatsResult>, MyError> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(_) = field.file_name() {
            let data = field.bytes().await.unwrap();

            let result = analyze_zip_bytes(&data)?;
            return Ok(Json(result));
        }
    }

    Err(MyError::NotFound)
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();
}


#[tokio::main]
async fn main() {
    init_tracing();

    let cors = CorsLayer::new()
        .allow_origin(Any)   // 👈 allows ALL origins
        .allow_methods(Any)  // GET, POST, PUT, etc.
        .allow_headers(Any); // any headers (Content-Type, etc.)

    let app = Router::new()
        .route("/upload", post(process_upload))
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    info!("Server is running on port 3000");

    axum::serve(listener, app).await.unwrap();
}
