use serde::{Deserialize, Serialize};

/// Configuration for Android devices.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AndroidConfig {
    collapse_key: Option<String>,
    priority: Option<String>,
    ttl: Option<String>,
}

impl AndroidConfig {
    #[must_use]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    #[must_use]
    pub fn collapse_key(&self) -> Option<&String> {
        self.collapse_key.as_ref()
    }

    #[must_use]
    pub fn priority(&self) -> Option<&String> {
        self.priority.as_ref()
    }

    #[must_use]
    pub fn ttl(&self) -> Option<&String> {
        self.ttl.as_ref()
    }

    pub fn set_collapse_key(&mut self, collapse_key: Option<String>) {
        self.collapse_key = collapse_key;
    }

    pub fn set_priority(&mut self, priority: Option<String>) {
        self.priority = priority;
    }

    pub fn set_ttl(&mut self, ttl: Option<String>) {
        self.ttl = ttl;
    }
}
