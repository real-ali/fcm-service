use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(clippy::enum_variant_names)]
pub enum Proxy {
    #[default]
    ProxyUnspecified,
    Allow,
    Deny,
    IfPriorityLowered,
}
