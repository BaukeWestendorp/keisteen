use std::collections::HashMap;

use uuid::Uuid;

use crate::server::player::Player;

pub struct PlayerList {
    max_players: usize,
    players: HashMap<Uuid, Player>,
}

impl PlayerList {
    pub fn new(max_players: usize) -> Self {
        Self { max_players, players: HashMap::new() }
    }

    pub fn max_players(&self) -> usize {
        self.max_players
    }

    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    pub fn is_empty(&self) -> bool {
        self.players.is_empty()
    }

    pub fn players(&self) -> impl Iterator<Item = &Player> {
        self.players.values()
    }

    pub fn get(&self, uuid: &Uuid) -> Option<&Player> {
        self.players.get(uuid)
    }

    pub fn get_mut(&mut self, uuid: &Uuid) -> Option<&mut Player> {
        self.players.get_mut(uuid)
    }

    pub fn add(&mut self, player: Player) {
        self.players.insert(player.uuid(), player);
    }

    pub fn remove(&mut self, uuid: &Uuid) -> Option<Player> {
        self.players.remove(uuid)
    }
}
