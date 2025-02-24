use serde::{Deserialize, Serialize};

/// Additional FCM options.
#[derive(Serialize, Deserialize, Default)]
pub struct FcmOptions {
    analytics_label: Option<String>,
}

impl FcmOptions {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn analytics_label(&self) -> Option<&String> {
        self.analytics_label.as_ref()
    }

    pub fn set_analytics_label(&mut self, analytics_label: Option<String>) {
        self.analytics_label = analytics_label;
    }
}
