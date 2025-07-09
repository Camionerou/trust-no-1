use bevy::prelude::*;
use tn1_shared::events::*;
use tn1_shared::components::*;

pub struct PositionReceiverPlugin;

impl Plugin for PositionReceiverPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LocalPlayerId(None))
            .insert_resource(LastSeenPlayers::default())
            .add_systems(Update, (
                receive_player_positions,
                update_camera_from_local_player,
                cleanup_disconnected_players,
            ));
    }
}

#[derive(Resource)]
pub struct LocalPlayerId(pub Option<PlayerId>);

#[derive(Resource, Default)]
pub struct LastSeenPlayers {
    pub players: std::collections::HashSet<PlayerId>,
    pub last_update: f32,
}

/// Recibe las posiciones autoritativas del servidor y las aplica
fn receive_player_positions(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut position_events: EventReader<PlayerPositionEvent>,
    mut player_query: Query<(&mut Transform, &PlayerId), With<Player>>,
    local_player_query: Query<&PlayerId, With<LocalPlayer>>,
    mut local_player_id: ResMut<LocalPlayerId>,
    mut last_seen: ResMut<LastSeenPlayers>,
) {
    // Determinar cuál es nuestro jugador local
    if local_player_id.0.is_none() {
        if let Ok(local_id) = local_player_query.get_single() {
            local_player_id.0 = Some(*local_id);
        }
    }

    for position_event in position_events.read() {
        // Marcar este jugador como visto
        last_seen.players.insert(position_event.player_id);
        
        // Verificar si es nuestro jugador local
        let is_local_player = local_player_id.0
            .map(|local_id| local_id == position_event.player_id)
            .unwrap_or(false);

        // Buscar el jugador correspondiente
        let mut found_player = false;
        for (mut transform, player_id) in player_query.iter_mut() {
            if *player_id == position_event.player_id {
                // Solo actualizar jugadores remotos, no el local
                if !is_local_player {
                    transform.translation = position_event.position;
                    transform.rotation = position_event.rotation;
                }
                found_player = true;
                break;
            }
        }

        // Si no encontramos el jugador y no es el local, crear uno nuevo (jugador remoto)
        if !found_player && !is_local_player {
            info!("🌐 Creando jugador remoto: {:?}", position_event.player_id);
            
            // Crear representación visual del jugador remoto
            let sphere_mesh = meshes.add(Sphere::new(0.5).mesh().ico(5).unwrap());
            let remote_player_material = materials.add(Color::srgb(1.0, 0.3, 0.3)); // ROJO para jugadores remotos
            
            commands.spawn((
                Mesh3d(sphere_mesh),
                MeshMaterial3d(remote_player_material),
                Transform::from_translation(position_event.position),
                Player,
                position_event.player_id,
                PlayerName(format!("Remote_{}", position_event.player_id.0.to_string()[..8].to_string())),
                Health::new(100.0),
            ));
        }
    }
}

/// Actualiza la cámara para seguir al jugador local
fn update_camera_from_local_player(
    local_player_query: Query<&Transform, (With<LocalPlayer>, Without<Camera3d>)>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
) {
    if let Ok(player_transform) = local_player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            // Posicionar cámara en la cabeza del jugador
            camera_transform.translation = player_transform.translation + 
                Vec3::new(0.0, 1.6, 0.0); // Altura de los ojos
        }
    }
}

/// Limpia jugadores remotos que se han desconectado
fn cleanup_disconnected_players(
    mut commands: Commands,
    remote_players: Query<(Entity, &PlayerId), (With<Player>, Without<LocalPlayer>)>,
    mut last_seen: ResMut<LastSeenPlayers>,
    time: Res<Time>,
) {
    let current_time = time.elapsed().as_secs_f32();
    
    // Solo verificar cada 5 segundos
    if current_time - last_seen.last_update < 5.0 {
        return;
    }
    last_seen.last_update = current_time;
    
    // Remover jugadores que no hemos visto en un tiempo
    for (entity, player_id) in remote_players.iter() {
        if !last_seen.players.contains(player_id) {
            info!("🗑️ Removiendo jugador remoto desconectado: {:?}", player_id);
            commands.entity(entity).despawn();
        }
    }
    
    // Limpiar la lista para el siguiente ciclo
    last_seen.players.clear();
}