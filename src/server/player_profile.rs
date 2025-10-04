use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PlayerProfile {
    uuid: Uuid,
    username: String,
    // TODO: Implement player properties.
    properties: (),
}

impl PlayerProfile {
    pub fn new(uuid: Uuid, username: String) -> Self {
        Self { uuid, username, properties: () }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn properties(&self) -> () {
        self.properties
    }
}
