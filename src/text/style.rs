// TODO: This is very ad-hoc. Properly serialize fields with enums etc.
//       https://minecraft.wiki/w/Text_component_format#Java_Edition
#[derive(Debug, Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Style {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shadow_color: Option<String>,
}
