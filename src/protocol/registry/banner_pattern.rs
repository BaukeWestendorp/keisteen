use crate::protocol::registry::Registry;
use crate::types::Identifier;

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BannerPattern {
    // TODO: To make this more like the vanilla implementation in java, this should be an enum with more possible asset loaders.
    // TODO: This should really be a `ResourceLocation`.
    asset_id: Identifier,
    translation_key: String,
}

impl Registry for BannerPattern {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "banner_pattern").unwrap()
    }
}
