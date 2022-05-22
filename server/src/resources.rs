use bevy::ecs::entity::Entity;
use naia_bevy_server::{RoomKey, UserKey};
use shared::protocol::KeyCommand;
use std::collections::HashMap;

pub struct Global {
    pub main_room_key: RoomKey,
    pub user_to_prediction_map: HashMap<UserKey, Entity>,
    pub player_last_command: HashMap<Entity, KeyCommand>,
}
