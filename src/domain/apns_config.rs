use std::{collections::HashMap, hash::RandomState};

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Configuration for Apple Push Notification Service (APNs).
 #[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ApnsConfig {
    headers: Option<HashMap<String, String>>,
    payload: Option<HashMap<String, serde_json::Value>>,
}

impl ApnsConfig {
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
    pub fn payload(&self) -> Option<&HashMap<String, Value, RandomState>> {
        self.payload.as_ref()
    }

    pub fn set_headers(&mut self, headers: Option<HashMap<String, String>>) {
        self.headers = headers;
    }

    pub fn set_payload(&mut self, payload: Option<HashMap<String, serde_json::Value>>) {
        self.payload = payload;
    }
}
