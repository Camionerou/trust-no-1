use bevy::prelude::*;
use tn1_shared::{components::*, events::*};

pub struct ServerPhysicsPlugin;

impl Plugin for ServerPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_physics_world)
            .add_systems(Update, (
                process_player_inputs,
                update_player_physics,
                send_position_updates,
                validate_player_positions,
            ));
    }
}

fn setup_physics_world() {
    info!("Mundo físico simplificado inicializado");
    info!("Sistema de colisiones básico habilitado");
}

fn process_player_inputs(
    mut input_events: EventReader<PlayerInputEvent>,
    mut player_query: Query<(&mut PlayerController, &mut Transform, &PlayerId), With<Player>>,
    time: Res<Time>,
) {
    for event in input_events.read() {
        if let Ok((mut controller, _transform, _)) = 
            player_query.iter_mut().find(|(_, _, id)| **id == event.player_id).ok_or(()) {
            
            let input = &event.input;
            let dt = time.delta().as_secs_f32();
            
            // Constantes de movimiento
            const MOVE_SPEED: f32 = 7.0;
            const SPRINT_MULTIPLIER: f32 = 1.5;
            const JUMP_FORCE: f32 = 8.0;
            const MAX_SPEED: f32 = 20.0;
            
            // Calcular velocidad horizontal
            let mut horizontal_velocity = Vec3::ZERO;
            
            if input.move_forward {
                horizontal_velocity += Vec3::NEG_Z;
            }
            if input.move_backward {
                horizontal_velocity += Vec3::Z;
            }
            if input.move_left {
                horizontal_velocity += Vec3::NEG_X;
            }
            if input.move_right {
                horizontal_velocity += Vec3::X;
            }
            
            // Normalizar y aplicar velocidad
            if horizontal_velocity.length() > 0.0 {
                horizontal_velocity = horizontal_velocity.normalize();
                let speed = if input.sprint { MOVE_SPEED * SPRINT_MULTIPLIER } else { MOVE_SPEED };
                horizontal_velocity *= speed;
            }
            
            // Aplicar rotación de cámara a la velocidad horizontal
            let rotation = Quat::from_euler(EulerRot::YXZ, input.camera_yaw, 0.0, 0.0);
            horizontal_velocity = rotation * horizontal_velocity;
            
            // Actualizar velocidad horizontal
            controller.velocity.x = horizontal_velocity.x;
            controller.velocity.z = horizontal_velocity.z;
            
            // Salto con jump buffer
            if input.jump {
                controller.jump_timer = 0.15; // Jump buffer
            }
            
            // Decrementar jump timer
            if controller.jump_timer > 0.0 {
                controller.jump_timer -= dt;
                
                // Si está en el suelo, puede saltar
                if controller.is_grounded {
                    controller.velocity.y = JUMP_FORCE;
                    controller.jump_timer = 0.0;
                    controller.is_grounded = false;
                    info!("Player jumped! Velocity Y: {}", controller.velocity.y);
                }
            }
            
            // Limitar velocidad máxima
            let horizontal_speed = Vec2::new(controller.velocity.x, controller.velocity.z).length();
            if horizontal_speed > MAX_SPEED {
                let factor = MAX_SPEED / horizontal_speed;
                controller.velocity.x *= factor;
                controller.velocity.z *= factor;
            }
            
            // Anti-cheat: validar que la velocidad no sea imposible
            if controller.velocity.length() > 50.0 {
                warn!("Player {:?} velocity too high: {}, clamping", event.player_id, controller.velocity.length());
                controller.velocity = controller.velocity.normalize() * 20.0;
            }
        }
    }
}

fn update_player_physics(
    mut player_query: Query<(&mut Transform, &mut PlayerController), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, mut player_controller) in player_query.iter_mut() {
        let dt = time.delta().as_secs_f32();
        
        // Aplicar gravedad
        if !player_controller.is_grounded {
            player_controller.velocity.y -= 9.81 * dt;
        }
        
        // Actualizar posición
        transform.translation += player_controller.velocity * dt;
        
        // Detectar colisión con el suelo (simplificado)
        if transform.translation.y <= 0.0 {
            transform.translation.y = 0.0;
            if player_controller.velocity.y < 0.0 {
                player_controller.velocity.y = 0.0;
                player_controller.is_grounded = true;
            }
        } else {
            player_controller.is_grounded = false;
        }
        
        // Límites del mundo
        transform.translation.x = transform.translation.x.clamp(-50.0, 50.0);
        transform.translation.z = transform.translation.z.clamp(-50.0, 50.0);
    }
}

fn send_position_updates(
    player_query: Query<(&Transform, &PlayerId), With<Player>>,
    mut position_events: EventWriter<PlayerPositionEvent>,
) {
    for (transform, player_id) in player_query.iter() {
        position_events.send(PlayerPositionEvent {
            player_id: *player_id,
            position: transform.translation,
            rotation: transform.rotation,
        });
    }
}

fn validate_player_positions(
    mut player_query: Query<(&mut Transform, &PlayerId), With<Player>>,
) {
    for (mut transform, player_id) in player_query.iter_mut() {
        // Anti-cheat: validar que el jugador no esté fuera de los límites del mundo
        if transform.translation.y < -50.0 {
            warn!("Player {:?} fell out of world, respawning", player_id);
            transform.translation = Vec3::new(0.0, 5.0, 0.0);
        }
        
        // Validar que no esté demasiado lejos del centro
        let distance_from_center = transform.translation.xz().length();
        if distance_from_center > 100.0 {
            warn!("Player {:?} too far from center, teleporting back", player_id);
            transform.translation.x = transform.translation.x.clamp(-50.0, 50.0);
            transform.translation.z = transform.translation.z.clamp(-50.0, 50.0);
        }
    }
} 