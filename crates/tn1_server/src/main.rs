use bevy::prelude::*;
use bevy::app::ScheduleRunnerPlugin;
use std::time::Duration;
use tn1_shared::events::*;

mod physics;
mod world;
mod systems;
mod networking;
mod database;

use physics::ServerPhysicsPlugin;
use world::WorldPlugin;
use systems::SystemsPlugin;
use networking::NetworkingPlugin;
use database::DatabasePlugin;

fn main() {
    // Cargar variables de entorno
    dotenv::dotenv().ok();
    
    // Configurar logging
    let log_level = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "warn,tn1_server=info".to_string());
    tracing_subscriber::fmt()
        .with_env_filter(log_level)
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
            DatabasePlugin,
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