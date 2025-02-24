use serde::{Deserialize, Serialize};

/// Represents the notification content of an FCM message.
#[derive(Serialize, Deserialize, Default)]
pub struct FcmNotification {
    title: String,
    body: String,
    image: Option<String>,
}

impl FcmNotification {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn body(&self) -> &str {
        &self.body
    }

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
