OAuth FCM Library
=================

This library is designed to simplify the process of sending Firebase Cloud Messaging (FCM) messages. It
handles the OAuth token for you, ensuring that tokens are refreshed when expired, and provides
a simple interface for sending both FCM data and notification messages.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
oauth_fcm = "0.1.0"
```

## Usage

Simple example for axum. For a more detailed example look at the [axum example](./examples/axum_example.rs).

```rust
use oauth_fcm::{create_shared_token_manager, send_fcm_message, FcmNotification, SharedTokenManager};

#[derive(serde::Serialize)]
struct YourDataType {
    // Your data here
    key: String,
}

async fn send_notification_route(Extension(token_manager): Extension<SharedTokenManager>, ) {
    let data = YourDataType {
        key: "value".to_string(),
    };
    let notification = FcmNotification {
        title: "Title".to_string(),
        body: "Body".to_string(),
    };
    send_fcm_message("DEVICE_TOKEN", Some(notification), Some(data), &token_manager, "PROJECT_ID").await.unwrap();
}

#[tokio::main]
async fn main() {
    let shared_token_manager =
        create_shared_token_manager("path/to/google/credentials.json").expect("Could not find credentials.json");

    let app = Router::new()
        .route("/send", post(send_notification_route))
        .layer(Extension(shared_token_manager));

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", "127.0.0.1", "8080")).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
```

## Examples

For more detailed examples, please refer to the [Examples] directory in the repository. There you can find example
implementations for either [Rocket] or [Axum]. Feel free to open a merge request for any other framework.

[Rocket]: https://rocket.rs/
[Axum]: https://github.com/tokio-rs/axum

[Examples]: ./examples

# License

Licensed under [MIT license]

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, shall be licensed as MIT, without any additional terms or conditions.

[MIT license]: ./LICENSE
