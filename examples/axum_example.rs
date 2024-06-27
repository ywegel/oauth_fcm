use axum::{extract::Extension, routing::post, Router};
use oauth_fcm::{create_shared_token_manager, send_fcm_message, SharedTokenManager};
use serde::Serialize;
use std::fs::File;

#[derive(Serialize)]
struct MyData {
    message: String,
    count: i32,
}

async fn send_notification(
    Extension(token_manager): Extension<SharedTokenManager>,
) -> Result<String, String> {
    // It is a good idea to load these from an .env file. Additionally, you can store them in a shared `Config` state.
    let device_token = "YOUR_DEVICE_TOKEN";
    let project_id = "YOUR_PROJECT_ID";
    let data = MyData {
        message: "Hello from Axum!".to_string(),
        count: 42,
    };

    send_fcm_message(device_token, None, Some(data), &token_manager, project_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok("FCM message sent successfully".to_string())
}

#[tokio::main]
async fn main() {
    let shared_token_manager =
        create_shared_token_manager(File::open("path/to/google/credentials.json").unwrap())
            .expect("Could not find credentials.json");

    let app = Router::new()
        .route("/send", post(send_notification))
        .layer(Extension(shared_token_manager));

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", "127.0.0.1", "8080"))
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start axum server");
}
