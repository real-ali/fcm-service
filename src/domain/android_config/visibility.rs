use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(clippy::enum_variant_names)]
pub enum Visibility {
    #[default]
    VisibilityUnspecified,
    Private,
    Public,
    Secret,
}
