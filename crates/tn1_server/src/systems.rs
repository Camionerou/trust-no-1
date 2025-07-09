use bevy::prelude::*;
use tn1_shared::components::*;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_player_physics,
                process_player_input,
            ));
    }
}

fn update_player_physics(
    mut query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    // Física básica del servidor (por ahora vacía)
    let _delta = time.delta_secs();
    
    for (_transform, _player) in query.iter_mut() {
        // Aquí irá la simulación física autoritativa
    }
}

fn process_player_input(
    // Aquí procesaremos los inputs de los jugadores cuando tengamos networking
) {
    // Por ahora vacío
}