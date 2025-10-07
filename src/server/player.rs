use uuid::Uuid;

pub struct Player {
    uuid: Uuid,
    username: String,
}

impl Player {
    pub fn new(uuid: Uuid, username: String) -> Self {
        Self { uuid, username }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn username(&self) -> &str {
        &self.username
    }
}
