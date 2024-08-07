use std::fs::File;
use std::sync::Once;

use oauth_fcm::create_shared_token_manager;
use oauth_fcm::send_fcm_message_with_url;
use serde_json::json;

use crate::test_helpers::FcmBaseTest;
use crate::test_helpers::TestData;

mod test_helpers;

static TRACING: Once = Once::new();

#[tokio::test]
async fn successful_fcm_test() {
    // Output logs to the console
    TRACING.call_once(tracing_subscriber::fmt::init);

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
        .with_status(200)
        .create();

    let shared_token_manager =
        create_shared_token_manager(File::open("tests/mock_credentials.json").unwrap())
            .expect("Failed to create SharedTokenManager");

    {
        let mut guard = shared_token_manager.lock().await;

        assert!(guard.is_token_expired());

        // Get a valid first token from the mock instead of the real server
        guard
            .refresh_token_with_url(&base.mock_auth_url())
            .await
            .expect("Failed to refresh token");

        assert!(!guard.is_token_expired());
        assert!(guard.get_token().await.is_ok());
        assert!(!guard.get_token().await.unwrap().is_empty());
    }

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

    assert!(result.is_ok());

    mock_auth.assert_async().await;
    mock_fcm.assert_async().await;
}

#[tokio::test]
async fn successful_fcm_test_from_string() {
    // Output logs to the console
    TRACING.call_once(tracing_subscriber::fmt::init);

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
        .with_status(200)
        .create();

    let creds = std::fs::read_to_string("tests/mock_credentials.json").unwrap();
    let shared_token_manager =
        create_shared_token_manager(creds.as_bytes()).expect("Failed to create SharedTokenManager");

    {
        let mut guard = shared_token_manager.lock().await;

        assert!(guard.is_token_expired());

        // Get a valid first token from the mock instead of the real server
        guard
            .refresh_token_with_url(&base.mock_auth_url())
            .await
            .expect("Failed to refresh token");

        assert!(!guard.is_token_expired());
        assert!(guard.get_token().await.is_ok());
        assert!(!guard.get_token().await.unwrap().is_empty());
    }

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

    assert!(result.is_ok());

    mock_auth.assert_async().await;
    mock_fcm.assert_async().await;
}
