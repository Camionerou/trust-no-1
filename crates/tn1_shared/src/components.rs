use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct PlayerId(pub uuid::Uuid);

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }
}

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct PlayerName(pub String);

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default)]
pub struct LocalPlayer;

#[derive(Component, Default)]
pub struct PlayerController {
    pub velocity: Vec3,
    pub speed: f32,
    pub is_grounded: bool,
    pub jump_timer: f32,
}

impl PlayerController {
    pub fn new() -> Self {
        Self {
            velocity: Vec3::ZERO,
            speed: 7.0,
            is_grounded: false,
            jump_timer: 0.0,
        }
    }
}