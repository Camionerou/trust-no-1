use bevy::prelude::*;

pub struct InputSenderPlugin;

impl Plugin for InputSenderPlugin {
    fn build(&self, _app: &mut App) {
        // El envío de inputs ahora se maneja directamente en networking.rs
        // Este plugin queda vacío pero lo mantenemos por compatibilidad
    }
} 