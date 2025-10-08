use crate::mc::resources::ResourceLocation;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ClientAsset {
    pub id: ResourceLocation,
}
