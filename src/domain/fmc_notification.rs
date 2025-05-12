use serde::{Deserialize, Serialize};

/// Represents the notification content of an FCM message.
 #[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FcmNotification {
    title: String,
    body: String,
    image: Option<String>,
}

impl FcmNotification {
    #[must_use]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[must_use]
    pub fn body(&self) -> &str {
        &self.body
    }

    #[must_use]
    pub fn image(&self) -> Option<&String> {
        self.image.as_ref()
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_body(&mut self, body: String) {
        self.body = body;
    }

    pub fn set_image(&mut self, image: Option<String>) {
        self.image = image;
    }
}
