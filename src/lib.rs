mod error;
mod fcm;
mod token_manager;

pub use error::{FcmError, NetworkError};
pub use fcm::{send_fcm_message, send_fcm_message_with_url, FcmNotification};
pub use token_manager::{SharedTokenManager, TokenManager};

use tracing::{info, instrument};

/// Creates a new `SharedTokenManager`.
///
/// This function is a helper for creating a `SharedTokenManager` from a given Google credentials location.
/// It creates a new `TokenManager` and wraps it in an `Arc<Mutex<_>>` to allow shared, mutable access from multiple threads.
///
/// # Arguments
///
/// * `google_credentials_location` - A string slice that holds the path to the Google credentials JSON file.
///
/// # Returns
///
/// This function returns a `Result` that contains a `SharedTokenManager` if the `TokenManager` was created successfully, or an `FcmError` if the Google credentials could not be read or parsed.
///
/// # Example
///
/// ```rust no_run
/// use oauth_fcm::create_shared_token_manager;
///
/// # fn main() {
/// let shared_token_manager = create_shared_token_manager("path_to_google_credentials.json").expect("Failed to create SharedTokenManager");
/// # }
/// ```
#[instrument(level = "info")]
pub fn create_shared_token_manager(
    google_credentials_location: &str,
) -> Result<SharedTokenManager, FcmError> {
    info!("Creating shared token manager");
    let manager = TokenManager::new(google_credentials_location)?;
    Ok(std::sync::Arc::new(tokio::sync::Mutex::new(manager)))
}
