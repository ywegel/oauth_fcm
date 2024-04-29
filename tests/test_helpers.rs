#[derive(serde::Serialize)]
pub struct TestData {
    pub title: String,
    pub description: String,
}

#[allow(dead_code)]
pub struct FcmBaseTest {
    oauth_host: String,
    pub oauth_path: String,
    fcm_host: String,
    pub fcm_path: String,

    pub access_token: String,
    pub device_token: String,
}

impl FcmBaseTest {
    pub fn new(oauth_host: String, oauth_path: String, fcm_host: String, fcm_path: String) -> Self {
        Self {
            oauth_host,
            oauth_path,
            fcm_host,
            fcm_path,

            access_token: "mock_access_token".to_string(),
            device_token: "mock_device_token".to_string(),
        }
    }

    pub fn mock_auth_url(&self) -> String {
        format!("{}{}", &self.oauth_host, &self.oauth_path)
    }

    #[allow(dead_code)]
    pub fn mock_fcm_url(&self) -> String {
        format!("{}{}", &self.fcm_host, &self.fcm_path)
    }
}
