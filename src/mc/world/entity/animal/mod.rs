use crate::mc::core::ClientAsset;
use crate::mc::registries::Registry;
use crate::mc::resources::ResourceLocation;
use crate::mc::types::Identifier;
use crate::mc::world::entity::variant::SpawnPrioritySelectors;

pub use frog::*;
pub use wolf::*;

mod frog;
mod wolf;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CatVariant {
    pub asset_id: ClientAsset,
    pub spawn_conditions: SpawnPrioritySelectors,
}

impl Registry for CatVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "cat_variant").unwrap()
    }
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChickenVariant {
    pub asset_id: ClientAsset,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub spawn_conditions: SpawnPrioritySelectors,
}

impl Registry for ChickenVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "chicken_variant").unwrap()
    }
}

#[derive(Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ChickenModel {
    #[default]
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "cold")]
    Cold,
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CowVariant {
    pub asset_id: ClientAsset,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub spawn_conditions: SpawnPrioritySelectors,
}

impl Registry for CowVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "cow_variant").unwrap()
    }
}

#[derive(Debug, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum CowModel {
    #[default]
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "cold")]
    Cold,
    #[serde(rename = "warm")]
    Warm,
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PigVariant {
    pub asset_id: ResourceLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<PigVariantModelType>,
    pub spawn_conditions: SpawnPrioritySelectors,
}

impl Registry for PigVariant {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "pig_variant").unwrap()
    }
}

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PigVariantModelType {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "cold")]
    Cold,
}
