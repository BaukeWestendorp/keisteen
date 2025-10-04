use crate::protocol::registry::Registry;
use crate::types::Identifier;

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Dialog {
    // TODO: Implement Dialog Registry
}

impl Registry for Dialog {
    fn identifier() -> Identifier {
        Identifier::new("minecraft", "dialog").unwrap()
    }
}
