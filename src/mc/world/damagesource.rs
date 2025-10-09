use crate::mc::registries::Registry;
use crate::mc::types::Identifier;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DamageType {
    pub message_id: String,
    pub scaling: DamageScaling,
    pub exhaustion: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effects: Option<DamageEffects>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub death_message_type: Option<DeathMessageType>,
}

impl Registry for DamageType {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "damage_type").unwrap()
    }
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum DamageScaling {
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "when_caused_by_living_non_player")]
    WhenCausedByLivingNonPlayer,
    #[serde(rename = "always")]
    Always,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum DeathMessageType {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "fall_variants")]
    FallVariants,
    #[serde(rename = "intentional_game_design")]
    IntentionalGameDesign,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum DamageEffects {
    #[default]
    #[serde(rename = "hurt")]
    Hurt,
    #[serde(rename = "thorns")]
    Thorns,
    #[serde(rename = "drowning")]
    Drowning,
    #[serde(rename = "burning")]
    Burning,
    #[serde(rename = "poking")]
    Poking,
    #[serde(rename = "freezing")]
    Freezing,
}
