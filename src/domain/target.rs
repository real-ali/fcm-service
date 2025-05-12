use serde::{Deserialize, Serialize};

/// Represents the target of the FCM message (token, topic, or condition).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Target {
    Token(String),
    Topic(String),
    Condition(String),
}

impl Default for Target {
    fn default() -> Self {
        Self::Token(String::new())
    }
}
