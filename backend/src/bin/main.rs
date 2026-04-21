use tower_http::trace::TraceLayer;
use whatsapp_analyzer::{analyze_zip_bytes, models::{errors::MyError, result::WhatsResult}};
use axum::{
    routing::post,
    Router,
    body::Bytes,
    Json
};
use tracing_subscriber;
use tower_http::cors::{CorsLayer, Any};

async fn process_upload(bytes: Bytes) -> Result<Json<WhatsResult>, MyError>{
    let result = analyze_zip_bytes(&bytes)?;
    Ok(Json(result))
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();
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

    println!("Server is running on port 3000");

    axum::serve(listener, app).await.unwrap();
}