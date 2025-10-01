use crate::protocol::registry::Registry;
use crate::types::Identifier;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DamageType {
    message_id: String,
    scaling: String,
    exhaustion: f32,
    effects: Option<String>,
    death_message_type: Option<String>,
}

impl Registry for DamageType {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "damage_type").unwrap()
    }
}
