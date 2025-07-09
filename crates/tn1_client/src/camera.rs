use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use tn1_shared::components::LocalPlayer;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, (
                handle_mouse_input,
                update_camera_position,
            ));
    }
}

#[derive(Component)]
pub struct PlayerCamera {
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: f32,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            sensitivity: 0.005,
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        PlayerCamera::default(),
    ));
}

fn handle_mouse_input(
    mut camera_query: Query<&mut PlayerCamera>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut window) = window_query.get_single_mut() else { return };
    let Ok(mut camera) = camera_query.get_single_mut() else { return };

    // Capturar cursor con clic izquierdo
    if mouse_input.just_pressed(MouseButton::Left) && window.cursor_options.grab_mode == CursorGrabMode::None {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
        info!("Cursor capturado - Usa ESC para liberar");
    }

    // Liberar cursor con ESC
    if keyboard.just_pressed(KeyCode::Escape) && window.cursor_options.grab_mode != CursorGrabMode::None {
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
        info!("Cursor liberado - Haz clic en la ventana para volver a controlar la cámara");
    }

    // Procesar movimiento del mouse solo si el cursor está capturado
    if window.cursor_options.grab_mode != CursorGrabMode::None {
        for motion in mouse_motion.read() {
            camera.yaw -= motion.delta.x * camera.sensitivity;
            camera.pitch -= motion.delta.y * camera.sensitivity;
            camera.pitch = camera.pitch.clamp(-1.5, 1.5);
        }
    }
}

fn update_camera_position(
    mut camera_query: Query<(&mut Transform, &PlayerCamera)>,
    player_query: Query<&Transform, (With<LocalPlayer>, Without<PlayerCamera>)>,
) {
    let Ok((mut camera_transform, camera)) = camera_query.get_single_mut() else { return };
    
    if let Ok(player_transform) = player_query.get_single() {
        // Aplicar rotación de la cámara
        let rotation = Quat::from_euler(EulerRot::YXZ, camera.yaw, camera.pitch, 0.0);
        
        // Posicionar la cámara directamente en la posición del jugador + altura de ojos
        let eye_height = Vec3::new(0.0, 1.6, 0.0);
        camera_transform.translation = player_transform.translation + eye_height;
        camera_transform.rotation = rotation;
    } else {
        // Si no hay jugador local, usar posición por defecto
        camera_transform.translation = Vec3::new(0.0, 5.0, 10.0);
        camera_transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}