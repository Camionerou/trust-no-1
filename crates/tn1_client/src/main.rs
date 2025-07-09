use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
use tracing::info;
use tn1_shared::events::*;

mod camera;
mod player;
mod input;
mod ui;
mod input_sender;
mod position_receiver;
mod networking;
mod player_tags;

use camera::CameraPlugin;
use player::PlayerPlugin;
use input::InputPlugin;
use ui::UIPlugin;
use input_sender::InputSenderPlugin;
// use position_receiver::PositionReceiverPlugin; // Deshabilitado - ahora networking maneja todo
use networking::ClientNetworkingPlugin;
use player_tags::PlayerTagsPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "TRUST-NO-1".to_string(),
                        resolution: (1280.0, 720.0).into(),
                        present_mode: PresentMode::AutoVsync,
                        window_theme: Some(WindowTheme::Dark),
                        focused: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
        )
        .add_event::<PlayerInputEvent>()
        .add_event::<PlayerPositionEvent>()
        .add_event::<PlayerSpawnEvent>()
        .add_event::<PlayerDespawnEvent>()
        .add_plugins((
            CameraPlugin,
            PlayerPlugin,
            InputPlugin,
            InputSenderPlugin,
            // PositionReceiverPlugin, // Deshabilitado - ahora networking maneja todo
            ClientNetworkingPlugin,
            PlayerTagsPlugin,
            UIPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Iniciando TRUST-NO-1 Cliente");

    // Plano temporal para testing
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Luz básica
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::PI / 4.0,
            std::f32::consts::PI / 4.0,
            0.0,
        )),
    ));
    
    // Luz ambiente
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
    });
}