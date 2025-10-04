use std::ops::{Deref, DerefMut};

use crate::server::entity::Entity;
use crate::server::player_profile::PlayerProfile;

#[derive(Debug)]
pub struct Player {
    entity: Entity,

    profile: PlayerProfile,
}

impl Player {
    pub fn new(profile: PlayerProfile) -> Self {
        Self { entity: Entity::new(), profile }
    }

    pub fn profile(&self) -> &PlayerProfile {
        &self.profile
    }
}

impl Deref for Player {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}

impl DerefMut for Player {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
