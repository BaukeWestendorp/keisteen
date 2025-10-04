use crate::protocol::registry::Registry;
use crate::types::Identifier;

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DamageType {
    message_id: String,
    scaling: String,
    exhaustion: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    effects: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    death_message_type: Option<String>,
}

impl Registry for DamageType {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "damage_type").unwrap()
    }
}
