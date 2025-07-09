use serde::{Deserialize, Serialize};
use bevy::prelude::*;
use crate::components::PlayerId;

pub const DEFAULT_PORT: u16 = 7777;
pub const PROTOCOL_VERSION: u32 = 2;
pub const TICK_RATE: u32 = 60;

/// Mensajes que el cliente envía al servidor
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMessage {
    /// Cliente solicita conectarse
    Connect { 
        protocol_version: u32,
        player_name: String,
    },
    
    /// Input del jugador
    PlayerInput {
        sequence: u32, // Para reconciliación client-side
        input: PlayerInput,
    },
    
    /// Cliente se desconecta limpiamente
    Disconnect,
    
    /// Heartbeat/keepalive
    Ping { timestamp: f64 },
}

/// Mensajes que el servidor envía al cliente
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    /// Confirmación de conexión con datos del jugador
    Connected {
        player_id: PlayerId,
        tick_rate: u32,
    },
    
    /// Estado completo del mundo (snapshot)
    WorldState {
        tick: u32,
        players: Vec<PlayerState>,
        timestamp: f64,
    },
    
    /// Un jugador se conectó
    PlayerJoined {
        player_id: PlayerId,
        position: Vec3,
    },
    
    /// Un jugador se desconectó
    PlayerLeft {
        player_id: PlayerId,
    },
    
    /// Respuesta a ping
    Pong { timestamp: f64 },
    
    /// Error o rechazo de conexión
    ConnectionError { reason: String },
}

/// Estado completo de un jugador
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerState {
    pub player_id: PlayerId,
    pub position: Vec3,
    pub velocity: Vec3,
    pub rotation: Quat,
    pub health: f32,
    pub is_grounded: bool,
    pub last_input_sequence: u32,
}

/// Input del jugador
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PlayerInput {
    pub move_forward: bool,
    pub move_backward: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub jump: bool,
    pub sprint: bool,
    pub camera_yaw: f32,
    pub camera_pitch: f32,
}