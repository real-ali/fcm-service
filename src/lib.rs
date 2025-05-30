use std::{error::Error, fs, io, path::PathBuf};

use gcp_auth::{CustomServiceAccount, TokenProvider};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
mod domain;

pub use domain::{
    AndroidConfig, AndroidNotification, ApnsConfig, Color, FcmMessage, FcmNotification, FcmOptions,
    LightSettings, NotificationPriority, Priority, Proxy, Target, Visibility, WebpushConfig,
};

/// Wrapper struct for FCM payload, required by the FCM v1 API.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FcmPayload {
    pub message: FcmMessage,
}

pub struct FcmService {
    pub credential_file: String,
}

impl FcmService {
    pub fn new(credential_file: impl Into<String>) -> Self {
        Self {
            credential_file: credential_file.into(),
        }
    }
}

/// Service for sending Firebase Cloud Messaging (FCM) notifications using the v1 API.
///
/// This service uses a Google Cloud service account credential file to authenticate
/// and send notifications to FCM.
///
/// # Examples
/// ```rust,no_run
/// use fcm_service::{FcmService, FcmMessage, FcmNotification, Target};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let service = FcmService::new("path/to/service-account.json");
///
///     let mut message = FcmMessage::new();
///     let mut notification = FcmNotification::new();
///     notification.set_title("Hello".to_string());
///     notification.set_body("World".to_string());
///     notification.set_image(None);
///     message.set_notification(Some(notification));
///     message.set_target(Target::Token("device-token".to_string()));
///
///     service.send_notification(message).await?;
///     Ok(())
/// }
/// ```
impl FcmService {
    /// Extracts the project ID from the service account credential file.
    fn get_project_id(&self) -> Result<String, Box<dyn Error>> {
        let content = fs::read_to_string(&self.credential_file)?;
        let json: Value = serde_json::from_str(&content)?;

        json.get("project_id")
            .and_then(|v| v.as_str())
            .map(std::string::ToString::to_string)
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "project_id not found").into()
            })
    }
    /// Sends an FCM notification asynchronously.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The credential file cannot be read or parsed
    /// - Authentication with GCP fails
    /// - The HTTP request to FCM fails
    /// - The FCM API returns an unsuccessful status
    pub async fn send_notification(&self, message: FcmMessage) -> Result<(), Box<dyn Error>> {
        let project_id = self.get_project_id()?;
        let client = Client::new();
        let credentials_path = PathBuf::from(&self.credential_file);
        // let service_account = CustomServiceAccount::from_file(credentials_path)?;
        let service_account = CustomServiceAccount::from_file(credentials_path)?;
        let scopes = &["https://www.googleapis.com/auth/firebase.messaging"];
        let token = service_account.token(scopes).await?;
        let url = format!("https://fcm.googleapis.com/v1/projects/{project_id}/messages:send");

        let payload = FcmPayload { message };

        let response = client
            .post(&url)
            .bearer_auth(token.as_str())
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            response.text().await?;

            Ok(())
        } else {
            let error_text = response.text().await?;
            Err(format!("Failed to send notification: {error_text:#?}").into())
        }
    }
}
#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use tempfile;

    use super::*;

    fn setup_dummy_credentials(temp_dir: &tempfile::TempDir) -> String {
        let credential_path = temp_dir.path().join("service-account.json");
        let mut file = File::create(&credential_path).unwrap();
        writeln!(
            file,
            r#"{{"project_id": "test-project", "client_email": "test@example.com"}}"#
        )
        .unwrap();
        credential_path.to_str().unwrap().to_string()
    }

    #[test]
    fn test_new_service() {
        let service = FcmService::new("dummy.json");
        assert_eq!(service.credential_file, "dummy.json");
    }

    #[test]
    fn test_get_project_id_success() {
        let temp_dir = tempfile::tempdir().unwrap();
        let credential_file = setup_dummy_credentials(&temp_dir);
        let service = FcmService::new(credential_file);
        let project_id = service.get_project_id().unwrap();
        assert_eq!(project_id, "test-project");
    }

    #[test]
    fn test_get_project_id_missing_file() {
        let service = FcmService::new("nonexistent.json");
        let result = service.get_project_id();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err().downcast_ref::<io::Error>(),
            Some(err) if err.kind() == io::ErrorKind::NotFound
        ));
    }

    #[test]
    fn test_get_project_id_invalid_json() {
        let temp_dir = tempfile::tempdir().unwrap();
        let credential_path = temp_dir.path().join("service-account.json");
        let mut file = File::create(&credential_path).unwrap();
        writeln!(file, "invalid json").unwrap();
        let service = FcmService::new(credential_path.to_str().unwrap());
        let result = service.get_project_id();
        assert!(result.is_err());
    }
}
