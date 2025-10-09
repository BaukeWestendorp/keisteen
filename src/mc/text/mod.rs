#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TextComponent {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    translate: Option<String>,
}

impl TextComponent {
    pub fn text(text: impl Into<String>) -> Self {
        Self { text: Some(text.into()), color: None, italic: None, translate: None }
    }
}
