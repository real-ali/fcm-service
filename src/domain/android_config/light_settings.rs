use serde::{Deserialize, Serialize, Serializer};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Color {
    #[serde(serialize_with = "serialize_clamped_f32")]
    pub red: f32,
    #[serde(serialize_with = "serialize_clamped_f32")]
    pub green: f32,
    #[serde(serialize_with = "serialize_clamped_f32")]
    pub blue: f32,
    #[serde(serialize_with = "serialize_clamped_f32")]
    pub alpha: f32,
}
impl Default for Color {
    fn default() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }
}
fn serialize_clamped_f32<S>(value: &f32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if *value < 0.0 || *value > 1.0 {
        return Err(serde::ser::Error::custom(format!(
            "Color value {} must be between 0.0 and 1.0",
            value
        )));
    }
    serializer.serialize_f32(*value)
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct LightSettings {
    pub color: Color,
    pub light_on_duration: Option<String>,
    pub light_off_duration: Option<String>,
}
