OAuth FCM Library
=================

[<img alt="crates.io" src="https://img.shields.io/crates/v/oauth_fcm">](https://crates.io/crates/oauth_fcm)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/ywegel/oauth_fcm/pull_request.yml)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/oauth_fcm">](https://docs.rs/oauth_fcm)
![GitHub License](https://img.shields.io/github/license/ywegel/oauth_fcm)

This library is designed to simplify the process of sending Firebase Cloud Messaging (FCM) messages. It
handles the OAuth token for you, ensuring that tokens are refreshed when expired, and provides
a simple interface for sending both FCM data and notification messages.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
oauth_fcm = "0.3.0"
```

## Usage

Simple example for axum. More detailed examples for other frameworks can be found in
the [examples' folder](./examples).

```rust
use std::fs::File;
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
        create_shared_token_manager(File::open("path/to/google/credentials.json")).expect("Could not find credentials.json");

    let app = Router::new()
        .route("/send", post(send_notification_route))
        .layer(Extension(shared_token_manager));

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", "127.0.0.1", "8080")).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
```

## Where to get your FCM credentials

1. Create a firebase project or use an existing one from the [firebase console](https://console.firebase.google.com/)
2. Head to the firebase project settings and select the "Service accounts" tab
3. Click on "All service accounts" with the Google cloud icon next to it. You will be redirected to the Google cloud
   console
4. In the cloud console select "Service Accounts" in the drawer menu on the left
5. There you select your firebase service account, which is
   named: `firebase-adminsdk-xyz@your-project.iam.gserviceaccount.com`
6. Click on the "keys" tab and create a new key under "add key" in the json format. It will be downloaded
7. Now you can use it in your project by providing the file location to your TokenManager from an env file or the file
   path:
   ``` rust
   let shared_token_manager =
        create_shared_token_manager(File::open("location/your-key-name-xyz.json").unwrap())
            .expect("Failed to create SharedTokenManager");
   ```
8. (Optional) It is better to not keep the file in your version control. Add this to your .gitignore
   file: `your-key-name-*.json`

## Examples

For more detailed examples, please refer to the [Examples] directory in the repository. There you can find example
implementations for either [Rocket] or [Axum]. Feel free to open a merge request for any other framework.

[Rocket]: https://rocket.rs/

[Axum]: https://github.com/tokio-rs/axum

[Examples]: ./examples

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on contributing. By contributing, you agree to license your
code under the project's [license](./LICENSE).

# License

Licensed under [MIT license]

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, shall be licensed as MIT, without any additional terms or conditions.

[MIT license]: ./LICENSE
