// TODO: This is very ad-hoc. Properly serialize fields with enums etc.
//       https://minecraft.wiki/w/Text_component_format#Java_Edition
#[derive(Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Style {
    pub color: Option<String>,
    pub font: Option<String>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    pub shadow_color: Option<String>,
}
