// Physics
pub const GRAVITY: f32 = -9.81;
pub const PLAYER_SPEED: f32 = 5.0;
pub const PLAYER_SPRINT_MULTIPLIER: f32 = 2.0;
pub const PLAYER_JUMP_VELOCITY: f32 = 7.0;
pub const JUMP_BUFFER_TIME: f32 = 0.15; // Tiempo para buffer de salto
pub const COYOTE_TIME: f32 = 0.1; // Tiempo para salto después de dejar el suelo

// Player
pub const PLAYER_HEIGHT: f32 = 1.8;
pub const PLAYER_RADIUS: f32 = 0.3;
pub const PLAYER_MASS: f32 = 80.0;
pub const PLAYER_MAX_HEALTH: f32 = 100.0;

// Camera
pub const CAMERA_SENSITIVITY: f32 = 0.005; // Aumentado para trackpad
pub const CAMERA_FOV: f32 = 90.0;
pub const CAMERA_HEIGHT_OFFSET: f32 = 1.6;

// Network
pub const SERVER_TICK_RATE: f32 = 64.0;
pub const CLIENT_UPDATE_RATE: f32 = 60.0;