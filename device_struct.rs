use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insets {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Orientation {
    pub width: i32,
    pub height: i32,
    #[serde(default)]
    pub outline: Option<Outline>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outline {
    pub image: Option<String>,
    pub insets: Option<Insets>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Screen {
    #[serde(rename = "device-pixel-ratio")]
    pub device_pixel_ratio: f64,
    pub horizontal: Orientation,
    pub vertical: Orientation,
    #[serde(rename = "vertical-spanned", skip_serializing_if = "Option::is_none")]
    pub vertical_spanned: Option<Orientation>,
    #[serde(rename = "horizontal-spanned", skip_serializing_if = "Option::is_none")]
    pub horizontal_spanned: Option<Orientation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAgentMetadata {
    pub platform: String,
    #[serde(rename = "platformVersion")]
    pub platform_version: String,
    pub architecture: String,
    pub model: String,
    pub mobile: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmulatedDevice {
    pub title: String,
    #[serde(rename = "type")]
    pub device_type: String, // "phone", "tablet", etc.
    pub order: i32,
    #[serde(rename = "user-agent")]
    pub user_agent: String,
    pub capabilities: Vec<String>, // ["touch", "mobile"]
    pub screen: Screen,
    #[serde(rename = "user-agent-metadata", skip_serializing_if = "Option::is_none")]
    pub user_agent_metadata: Option<UserAgentMetadata>,
    #[serde(rename = "show-by-default")]
    pub show_by_default: bool,
    #[serde(rename = "dual-screen", default)]
    pub dual_screen: bool,
    #[serde(rename = "foldable-screen", default)]
    pub foldable_screen: bool,
}

// A collection of these devices would be Vec<EmulatedDevice>
