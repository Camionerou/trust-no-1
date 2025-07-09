use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Startup, setup_physics_world)
            .add_systems(Update, debug_physics_toggle);
    }
}

fn setup_physics_world(mut commands: Commands) {
    info!("Configurando mundo físico...");
    
    // Crear el suelo
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(50.0, 0.5, 50.0),
        Transform::from_xyz(0.0, -0.5, 0.0),
        Name::new("Suelo"),
    ));
    
    // Crear algunas paredes
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(0.5, 5.0, 50.0),
        Transform::from_xyz(50.0, 5.0, 0.0),
        Name::new("Pared Este"),
    ));
    
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(0.5, 5.0, 50.0),
        Transform::from_xyz(-50.0, 5.0, 0.0),
        Name::new("Pared Oeste"),
    ));
    
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(50.0, 5.0, 0.5),
        Transform::from_xyz(0.0, 5.0, 50.0),
        Name::new("Pared Norte"),
    ));
    
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(50.0, 5.0, 0.5),
        Transform::from_xyz(0.0, 5.0, -50.0),
        Name::new("Pared Sur"),
    ));
    
    // Crear algunos objetos interactivos
    for i in 0..5 {
        commands.spawn((
            RigidBody::Dynamic,
            Collider::cuboid(0.5, 0.5, 0.5),
            Transform::from_xyz(i as f32 * 2.0 - 4.0, 5.0, 0.0),
            Name::new(format!("Cubo {}", i)),
        ));
    }
}

fn debug_physics_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    mut debug_render_context: ResMut<DebugRenderContext>,
) {
    if keys.just_pressed(KeyCode::F4) {
        debug_render_context.enabled = !debug_render_context.enabled;
        info!("Debug de física: {}", if debug_render_context.enabled { "ON" } else { "OFF" });
    }
} 