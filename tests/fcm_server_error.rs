use serde_json::json;
use std::fs::File;

use oauth_fcm::{create_shared_token_manager, send_fcm_message_with_url, FcmError, NetworkError};

use crate::test_helpers::{FcmBaseTest, TestData};

mod test_helpers;

#[tokio::test]
async fn test_fcm_server_error() {
    // Output logs to the console
    tracing_subscriber::fmt::init();

    let mut server = mockito::Server::new_async().await;

    let project_id = "mock_project_id";
    let base = FcmBaseTest::new(
        server.url(),
        "/token".to_string(),
        server.url(),
        format!("/v1/projects/{}/messages:send", project_id),
    );

    let mock_auth = server
        .mock("POST", base.oauth_path.as_str())
        .with_status(200)
        .with_body(
            json!({
                "access_token": base.access_token,
                "scope": "https://www.googleapis.com/auth/prediction",
                "token_type": "Bearer",
                "expires_in": 3600,
            })
            .to_string(),
        )
        .create();

    let mock_fcm = server
        .mock("POST", base.fcm_path.as_str())
        .with_status(500)
        .with_body("Internal Server Error")
        .create();

    let shared_token_manager =
        create_shared_token_manager(File::open("tests/mock_credentials.json").unwrap())
            .expect("Failed to create SharedTokenManager");
    // Force refresh with valid url
    shared_token_manager
        .lock()
        .await
        .refresh_token_with_url(&base.mock_auth_url())
        .await
        .expect("Failed to refresh token");

    let data = TestData {
        title: "Test title".to_string(),
        description: "Test description".to_string(),
    };

    let result = send_fcm_message_with_url(
        &base.device_token,
        None,
        Some(data),
        &shared_token_manager,
        &base.mock_fcm_url(),
    )
    .await;

    assert!(result.is_err());
    let network_error = result.unwrap_err();
    eprintln!("network_error: {:?}", network_error);
    assert!(matches!(
        network_error,
        FcmError::FcmNetworkError(NetworkError::ServerError(500, _))
    ));

    mock_auth.assert_async().await;
    mock_fcm.assert_async().await;
}
