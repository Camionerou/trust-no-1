use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WorldSettings::default())
            .add_systems(Startup, initialize_world)
            .add_systems(Update, update_world_time);
    }
}

#[derive(Resource)]
pub struct WorldSettings {
    pub world_size: f32,
    pub chunk_size: f32,
    pub time_scale: f32,
    pub current_time: f32,
}

impl Default for WorldSettings {
    fn default() -> Self {
        Self {
            world_size: 5000.0,  // 5km x 5km
            chunk_size: 100.0,   // chunks de 100x100 metros
            time_scale: 1.0,
            current_time: 12.0,  // Mediodía
        }
    }
}

fn initialize_world(world_settings: Res<WorldSettings>) {
    info!(
        "Mundo inicializado: {}x{} metros, chunks de {}m",
        world_settings.world_size,
        world_settings.world_size,
        world_settings.chunk_size
    );
}

fn update_world_time(
    mut world_settings: ResMut<WorldSettings>,
    time: Res<Time>,
) {
    // Actualizar tiempo del mundo (24 horas = 1 hora real por defecto)
    world_settings.current_time += time.delta_secs() * world_settings.time_scale / 3600.0;
    
    if world_settings.current_time >= 24.0 {
        world_settings.current_time -= 24.0;
    }
}