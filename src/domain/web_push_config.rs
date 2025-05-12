use std::{collections::HashMap, hash::RandomState};

use serde::{Deserialize, Serialize};

/// Configuration for web push notifications.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct WebpushConfig {
    headers: Option<HashMap<String, String>>,
    data: Option<HashMap<String, String>>,
}

impl WebpushConfig {
    #[must_use]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    #[must_use]
    pub fn headers(&self) -> Option<&HashMap<String, String, RandomState>> {
        self.headers.as_ref()
    }

    #[must_use]
    pub fn data(&self) -> Option<&HashMap<String, String, RandomState>> {
        self.data.as_ref()
    }

    pub fn set_headers(&mut self, headers: Option<HashMap<String, String>>) {
        self.headers = headers;
    }

    pub fn set_data(&mut self, data: Option<HashMap<String, String>>) {
        self.data = data;
    }
}
