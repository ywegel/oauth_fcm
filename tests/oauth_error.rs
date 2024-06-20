use std::fs::File;

use oauth_fcm::create_shared_token_manager;

use crate::test_helpers::FcmBaseTest;

mod test_helpers;

#[tokio::test]
async fn failing_oauth_token_refresh() {
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
        .with_status(400)
        .create();

    let shared_token_manager =
        create_shared_token_manager(File::open("tests/mock_credentials.json").unwrap())
            .expect("Failed to create SharedTokenManager");

    let res = {
        let mut guard = shared_token_manager.lock().await;

        assert!(guard.is_token_expired());

        // Get a valid first token from the mock instead of the real server
        guard.refresh_token_with_url(&base.mock_auth_url()).await
    };

    assert!(res.is_err());
    let error = res.unwrap_err();
    println!("Error: {}", error);

    mock_auth.assert_async().await;
}
