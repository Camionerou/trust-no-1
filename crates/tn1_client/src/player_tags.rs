use bevy::prelude::*;
use bevy::text::*;
use bevy::sprite::Anchor;
use tn1_shared::components::*;

pub struct PlayerTagsPlugin;

impl Plugin for PlayerTagsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            create_tags_for_new_players,
            update_tag_positions,
        ));
    }
}

#[derive(Component)]
pub struct PlayerTag {
    pub player_entity: Entity,
}

#[derive(Component)]
pub struct TagText;

#[derive(Component)]
pub struct Billboard;

fn create_tags_for_new_players(
    mut commands: Commands,
    // Jugadores remotos (sin LocalPlayer)
    remote_players: Query<(Entity, &PlayerId), (With<Player>, Without<LocalPlayer>, Without<PlayerTag>)>,
    // Jugador local
    local_players: Query<(Entity, &PlayerId), (With<LocalPlayer>, Without<PlayerTag>)>,
) {
    // Crear tags para jugadores remotos
    for (entity, player_id) in remote_players.iter() {
        // Crear texto 3D para jugador remoto
        let tag_text = format!("Player {}", &player_id.0.to_string()[..8]);
        
        // Agregar el componente PlayerTag directamente al jugador
        commands.entity(entity).insert(PlayerTag { player_entity: entity });
        
        // Crear la entidad de texto como hijo
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                Text2d::new(tag_text),
                TextLayout::new_with_justify(JustifyText::Center),
                Transform::from_translation(Vec3::new(0.0, 1.5, 0.0))
                    .with_scale(Vec3::splat(0.01)),
                TextColor(Color::srgb(1.0, 0.3, 0.3)), // Rojo para remotos
                TagText,
                Billboard,
            ));
        });
        
        info!("🏷️ Tag creado para jugador remoto: {:?}", player_id);
    }

    // Crear tags para el jugador local
    for (entity, player_id) in local_players.iter() {
        // Crear texto 3D para jugador local
        let tag_text = format!("You ({})", &player_id.0.to_string()[..8]);
        
        // Agregar el componente PlayerTag directamente al jugador
        commands.entity(entity).insert(PlayerTag { player_entity: entity });
        
        // Crear la entidad de texto como hijo
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                Text2d::new(tag_text),
                TextLayout::new_with_justify(JustifyText::Center),
                Transform::from_translation(Vec3::new(0.0, 1.5, 0.0))
                    .with_scale(Vec3::splat(0.01)),
                TextColor(Color::srgb(0.2, 0.6, 1.0)), // Azul para local
                TagText,
                Billboard,
            ));
        });
        
        info!("🏷️ Tag creado para jugador local: {:?}", player_id);
    }
}

fn update_tag_positions(
    mut tag_query: Query<&mut Transform, (With<TagText>, Without<Camera3d>)>,
    camera_query: Query<&Transform, With<Camera3d>>,
) {
    let Ok(camera_transform) = camera_query.get_single() else { return };

    // Hacer que todos los tags miren hacia la cámara (billboard effect)
    for mut tag_transform in tag_query.iter_mut() {
        // Mantener la posición relativa pero rotar hacia la cámara
        let look_at = camera_transform.translation;
        let up = Vec3::Y;
        
        // Calcular la rotación para mirar hacia la cámara
        let forward = (look_at - tag_transform.translation).normalize();
        let right = up.cross(forward).normalize();
        let up = forward.cross(right);
        
        tag_transform.rotation = Quat::from_mat3(&Mat3::from_cols(right, up, forward));
    }
} 