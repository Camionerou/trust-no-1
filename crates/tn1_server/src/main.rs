use bevy::prelude::*;
use bevy::app::ScheduleRunnerPlugin;
use std::time::Duration;
use tn1_shared::events::*;

mod physics;
mod world;
mod systems;
mod networking;

use physics::ServerPhysicsPlugin;
use world::WorldPlugin;
use systems::SystemsPlugin;
use networking::NetworkingPlugin;

fn main() {
    // Configurar logging - solo errores y warnings críticos
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();
    
    println!("🚀 Iniciando servidor Trust-No-1...");
    
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_secs_f32(1.0 / 60.0), // 60 TPS
        )))
        .add_event::<PlayerInputEvent>()
        .add_event::<PlayerPositionEvent>()
        .add_event::<PlayerSpawnEvent>()
        .add_event::<PlayerDespawnEvent>()
        .add_plugins((
            ServerPhysicsPlugin,
            NetworkingPlugin,
            WorldPlugin,
            SystemsPlugin,
        ))
        .add_systems(Startup, setup_server)
        .add_systems(Update, server_tick)
        .run();
}

fn setup_server() {
    println!("✅ Servidor Trust-No-1 iniciado correctamente");
    println!("🔧 Modo: Servidor autoritativo headless");
    println!("⚡ Física: Simplificada habilitada");
    println!("🔄 TPS: 60 (Ticks por segundo)");
    println!("🌐 Puerto: 7777 (TCP)");
    println!("📊 Logs: Solo errores y warnings");
}

fn server_tick(_time: Res<Time>, _server_state: Res<crate::networking::ServerState>) {
    // Sin logs periódicos - servidor silencioso
}