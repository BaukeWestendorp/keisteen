use uuid::Uuid;

use crate::mc::text::text_component::TextComponent;
use crate::server::player::Player;

#[derive(Debug)]
pub struct PlayerList {
    max_players: i32,

    players: Vec<Player>,
}

impl PlayerList {
    pub(crate) fn new(max_players: i32) -> Self {
        Self { max_players, players: Vec::new() }
    }

    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn get_player(&self, uuid: Uuid) -> Option<&Player> {
        self.players.iter().find(|p| p.profile().uuid() == uuid)
    }

    pub fn online_players(&self) -> i32 {
        self.players.len() as i32
    }

    pub fn max_players(&self) -> i32 {
        self.max_players
    }

    pub fn can_player_login(&self, uuid: Uuid) -> Result<(), TextComponent> {
        if self.get_player(uuid).is_some() {
            return Err(TextComponent {
                text: Some("You are already logged in.".to_string()),
                translate: None,
                color: None,
            });
        }

        if self.online_players() >= self.max_players() {
            return Err(TextComponent {
                text: Some("The server is full.".to_string()),
                translate: None,
                color: None,
            });
        }

        Ok(())
    }

    pub(crate) fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub(crate) fn remove_player(&mut self, uuid: Uuid) {
        self.players.retain(|p| p.profile().uuid() != uuid);
    }
}
