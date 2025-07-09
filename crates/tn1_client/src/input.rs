use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InputSettings>()
            .add_systems(Update, handle_debug_input);
    }
}

#[derive(Resource)]
pub struct InputSettings {
    pub mouse_sensitivity: f32,
    pub controller_sensitivity: f32,
}

impl Default for InputSettings {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 1.0,
            controller_sensitivity: 1.0,
        }
    }
}

fn handle_debug_input(
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // F3 para debug info (futuro)
    if keyboard.just_pressed(KeyCode::F3) {
        info!("Debug mode toggle");
    }
    
    // F11 para fullscreen (futuro)
    if keyboard.just_pressed(KeyCode::F11) {
        info!("Fullscreen toggle");
    }
}