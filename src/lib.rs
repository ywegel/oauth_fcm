#![forbid(unsafe_code)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::tests_outside_test_module,
    unused_qualifications,
    non_ascii_idents
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::future_not_send
)]

use std::fmt::Debug;
use std::io::Read;

pub use error::FcmError;
pub use error::NetworkError;
pub use fcm::send_fcm_message;
pub use fcm::send_fcm_message_with_url;
pub use fcm::FcmNotification;
pub use token_manager::SharedTokenManager;
pub use token_manager::TokenManager;
use tracing::info;
use tracing::instrument;

mod error;
mod fcm;
mod token_manager;

/// Creates a new `SharedTokenManager`.
///
/// This function is a helper for creating a `SharedTokenManager` from a given
/// Google credentials location. It creates a new `TokenManager` and wraps it in
/// an `Arc<Mutex<_>>` to allow shared, mutable access from multiple threads.
///
/// # Arguments
///
/// * `google_credentials_location` - A string slice that holds the path to the
///   Google credentials JSON file.
///
/// # Returns
///
/// This function returns a `Result` that contains a `SharedTokenManager` if the
/// `TokenManager` was created successfully, or an `FcmError` if the Google
/// credentials could not be read or parsed.
///
/// # Example
///
/// ```rust no_run
/// use std::fs::File;
///
/// use oauth_fcm::create_shared_token_manager;
///
/// # fn main() {
/// let shared_token_manager = create_shared_token_manager(File::open("path_to_google_credentials.json").expect("Failed to open file")).expect("Failed to create SharedTokenManager");
/// # }
/// ```
#[instrument(level = "info", skip_all)]
pub fn create_shared_token_manager<T: Read + Debug>(
    credentials: T,
) -> Result<SharedTokenManager, FcmError> {
    info!("Creating shared token manager");
    let manager = TokenManager::new(credentials)?;
    Ok(std::sync::Arc::new(tokio::sync::Mutex::new(manager)))
}
