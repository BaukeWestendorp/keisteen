use crate::mc::core::ClientAsset;
use crate::mc::registries::Registry;
use crate::mc::types::Identifier;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BannerPattern {
    pub asset_id: ClientAsset,
    pub translation_key: String,
}

impl Registry for BannerPattern {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "banner_pattern").unwrap()
    }
}
