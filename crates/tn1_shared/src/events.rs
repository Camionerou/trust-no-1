use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::PlayerId;
use crate::protocol::PlayerInput;

#[derive(Event, Serialize, Deserialize, Clone, Debug)]
pub struct PlayerSpawnEvent {
    pub player_id: PlayerId,
    pub position: Vec3,
    pub is_local: bool,
}

#[derive(Event, Serialize, Deserialize, Clone, Debug)]
pub struct PlayerDespawnEvent {
    pub entity: Entity,
    pub player_id: uuid::Uuid,
}

/// Evento que contiene los inputs del jugador enviados al servidor
#[derive(Event, Serialize, Deserialize, Clone, Debug)]
pub struct PlayerInputEvent {
    pub player_id: PlayerId,
    pub input: PlayerInput,
    pub timestamp: f64,
}

// PlayerInput ahora se importa desde protocol.rs

/// Evento que el servidor envía a los clientes con la posición autoritativa
#[derive(Event, Serialize, Deserialize, Clone, Debug)]
pub struct PlayerPositionEvent {
    pub player_id: PlayerId,
    pub position: Vec3,
    pub rotation: Quat,
}