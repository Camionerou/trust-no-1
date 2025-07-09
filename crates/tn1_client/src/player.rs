use bevy::prelude::*;
use tn1_shared::components::*;
use crate::networking::NetworkClient;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // El cliente ya no maneja física local ni spawning
        // Todo viene del servidor
        app.add_systems(Update, debug_player_info);
    }
}

fn debug_player_info(
    player_query: Query<(&Transform, &PlayerId), With<Player>>,
    local_player_query: Query<&PlayerId, With<LocalPlayer>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    client: Res<NetworkClient>,
) {
    if keyboard.just_pressed(KeyCode::F4) {
        info!("=== DEBUG INFO ===");
        info!("Conectado: {}", client.connected);
        info!("Jugador local: {:?}", client.local_player_id);
        
        if let Ok(local_id) = local_player_query.get_single() {
            info!("Entidad local encontrada: {:?}", local_id);
        }
        
        info!("Jugadores en escena:");
        for (transform, player_id) in player_query.iter() {
            let is_local = Some(*player_id) == client.local_player_id;
            info!("  {:?} - Pos: ({:.2}, {:.2}, {:.2}) - Local: {}", 
                player_id, 
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
                is_local
            );
        }
        
        info!("Estados conocidos:");
        for (id, state) in &client.player_states {
            info!("  {:?} - Pos: ({:.2}, {:.2}, {:.2}) - Vel: ({:.2}, {:.2}, {:.2})",
                id,
                state.position.x, state.position.y, state.position.z,
                state.velocity.x, state.velocity.y, state.velocity.z
            );
        }
        
        info!("==================");
    }
}