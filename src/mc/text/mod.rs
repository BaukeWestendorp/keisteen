#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TextComponent {
    text: String,
}

impl TextComponent {
    pub fn text(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}
