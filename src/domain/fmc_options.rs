use serde::{Deserialize, Serialize};

/// Additional FCM options.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FcmOptions {
    analytics_label: Option<String>,
}

impl FcmOptions {
    #[must_use]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    #[must_use]
    pub fn analytics_label(&self) -> Option<&String> {
        self.analytics_label.as_ref()
    }

    pub fn set_analytics_label(&mut self, analytics_label: Option<String>) {
        self.analytics_label = analytics_label;
    }
}
