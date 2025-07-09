use bevy::prelude::*;
use tn1_shared::{components::*, protocol::{self, *}};
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use std::io::{Read, Write};
use std::thread;
use std::collections::HashMap;

pub struct ClientNetworkingPlugin;

impl Plugin for ClientNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkClient::new())
            .insert_resource(InputSequence(0))
            .add_systems(Startup, connect_to_server)
            .add_systems(Update, (
                process_server_messages,
                send_player_input,
            ));
    }
}

#[derive(Resource)]
pub struct InputSequence(pub u32);

#[derive(Resource)]
pub struct NetworkClient {
    pub connected: bool,
    pub local_player_id: Option<PlayerId>,
    pub stream: Option<Arc<Mutex<TcpStream>>>,
    pub incoming_messages: Arc<Mutex<Vec<ServerMessage>>>,
    pub player_states: HashMap<PlayerId, PlayerState>,
}

impl NetworkClient {
    fn new() -> Self {
        Self {
            connected: false,
            local_player_id: None,
            stream: None,
            incoming_messages: Arc::new(Mutex::new(Vec::new())),
            player_states: HashMap::new(),
        }
    }
}

fn connect_to_server(mut client: ResMut<NetworkClient>) {
    info!("🔌 Conectando al servidor...");
    
    match TcpStream::connect(format!("127.0.0.1:{}", DEFAULT_PORT)) {
        Ok(stream) => {
            stream.set_nodelay(true).ok();
            
            let stream_arc = Arc::new(Mutex::new(stream));
            client.stream = Some(stream_arc.clone());
            
            // Enviar mensaje de conexión
            let connect_msg = ClientMessage::Connect {
                protocol_version: PROTOCOL_VERSION,
                player_name: "Jugador".to_string(), // TODO: configurar nombre
            };
            
            if let Ok(mut stream_lock) = stream_arc.lock() {
                send_client_message(&mut *stream_lock, &connect_msg);
            }
            
            // Thread para recibir mensajes
            let incoming = client.incoming_messages.clone();
            let stream_clone = stream_arc.clone();
            
            thread::spawn(move || {
                receive_server_messages(stream_clone, incoming);
            });
            
            info!("✅ Conectado al servidor");
        }
        Err(e) => {
            error!("❌ No se pudo conectar al servidor: {}", e);
        }
    }
}

fn receive_server_messages(
    stream: Arc<Mutex<TcpStream>>,
    incoming: Arc<Mutex<Vec<ServerMessage>>>,
) {
    let mut buffer = vec![0u8; 4096];
    let mut message_buffer = Vec::new();
    
    loop {
        let mut stream_lock = match stream.lock() {
            Ok(lock) => lock,
            Err(_) => break,
        };
        
        match stream_lock.read(&mut buffer) {
            Ok(0) => {
                info!("📤 Servidor desconectado");
                break;
            }
            Ok(n) => {
                message_buffer.extend_from_slice(&buffer[..n]);
                
                // Procesar mensajes completos
                while message_buffer.len() >= 4 {
                    let msg_len = u32::from_be_bytes([
                        message_buffer[0],
                        message_buffer[1],
                        message_buffer[2],
                        message_buffer[3],
                    ]) as usize;
                    
                    if message_buffer.len() >= 4 + msg_len {
                        // Mensaje completo
                        if let Ok(json_str) = std::str::from_utf8(&message_buffer[4..4 + msg_len]) {
                            if let Ok(msg) = serde_json::from_str::<ServerMessage>(json_str) {
                                incoming.lock().unwrap().push(msg);
                            }
                        }
                        
                        message_buffer.drain(0..4 + msg_len);
                    } else {
                        break;
                    }
                }
            }
            Err(e) => {
                if e.kind() != std::io::ErrorKind::WouldBlock {
                    error!("Error leyendo del servidor: {}", e);
                    break;
                }
            }
        }
        
        // Liberar el lock entre lecturas
        drop(stream_lock);
        thread::sleep(std::time::Duration::from_millis(1));
    }
}

fn process_server_messages(
    mut commands: Commands,
    mut client: ResMut<NetworkClient>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<(Entity, &PlayerId)>,
    mut transform_query: Query<&mut Transform>,
) {
    let messages: Vec<ServerMessage> = {
        let mut incoming = client.incoming_messages.lock().unwrap();
        std::mem::take(&mut *incoming)
    };
    
    for message in messages {
        match message {
            ServerMessage::Connected { player_id, tick_rate } => {
                client.connected = true;
                client.local_player_id = Some(player_id);
                info!("🎮 Conectado como jugador {:?}", player_id);
                info!("⚡ Tick rate del servidor: {} Hz", tick_rate);
            }
            
            ServerMessage::WorldState { players, .. } => {
                // Actualizar estados de jugadores
                for state in &players {
                    client.player_states.insert(state.player_id, state.clone());
                    
                    // Buscar entidad existente
                    let mut entity_found = None;
                    for (entity, id) in player_query.iter() {
                        if *id == state.player_id {
                            entity_found = Some(entity);
                            break;
                        }
                    }
                    
                    if let Some(entity) = entity_found {
                        // Actualizar transform existente
                        if let Ok(mut transform) = transform_query.get_mut(entity) {
                            transform.translation = state.position;
                            transform.rotation = state.rotation;
                        }
                    } else {
                        // Crear nueva entidad
                        let is_local = Some(state.player_id) == client.local_player_id;
                        
                        let mesh = meshes.add(Sphere::new(0.5).mesh().ico(5).unwrap());
                        let material = if is_local {
                            materials.add(Color::srgb(0.2, 0.6, 1.0)) // Azul para local
                        } else {
                            materials.add(Color::srgb(1.0, 0.2, 0.2)) // Rojo para remoto
                        };
                        
                        let mut entity_cmds = commands.spawn((
                            Mesh3d(mesh),
                            MeshMaterial3d(material),
                            Transform::from_translation(state.position)
                                .with_rotation(state.rotation),
                            Player,
                            state.player_id,
                        ));
                        
                        if is_local {
                            entity_cmds.insert(LocalPlayer);
                        }
                        
                        info!("🎯 Spawneado jugador {:?} (local: {})", state.player_id, is_local);
                    }
                }
                
                // Limpiar jugadores que ya no están
                let active_players: Vec<PlayerId> = players.iter().map(|p| p.player_id).collect();
                for (entity, player_id) in player_query.iter() {
                    if !active_players.contains(player_id) {
                        commands.entity(entity).despawn();
                        client.player_states.remove(player_id);
                    }
                }
            }
            
            ServerMessage::PlayerJoined { player_id, position } => {
                info!("👋 Jugador {:?} se unió al juego", player_id);
                
                // El jugador será creado cuando llegue el siguiente WorldState
                let state = PlayerState {
                    player_id,
                    position,
                    velocity: Vec3::ZERO,
                    rotation: Quat::IDENTITY,
                    health: 100.0,
                    is_grounded: true,
                    last_input_sequence: 0,
                };
                client.player_states.insert(player_id, state);
            }
            
            ServerMessage::PlayerLeft { player_id } => {
                info!("👋 Jugador {:?} abandonó el juego", player_id);
                
                // Despawnear entidad
                for (entity, id) in player_query.iter() {
                    if *id == player_id {
                        commands.entity(entity).despawn();
                        break;
                    }
                }
                client.player_states.remove(&player_id);
            }
            
            ServerMessage::ConnectionError { reason } => {
                error!("❌ Error de conexión: {}", reason);
                client.connected = false;
            }
            
            ServerMessage::Pong { .. } => {
                // Ignorar pongs por ahora
            }
        }
    }
}

fn send_player_input(
    client: Res<NetworkClient>,
    keyboard: Res<ButtonInput<KeyCode>>,
    camera_query: Query<&crate::camera::PlayerCamera>,
    mut sequence: ResMut<InputSequence>,
    _time: Res<Time>,
) {
    // Solo enviar si estamos conectados y tenemos un jugador
    if !client.connected || client.local_player_id.is_none() {
        return;
    }
    
    let Ok(camera) = camera_query.get_single() else { return };
    
    // Incrementar secuencia
    sequence.0 += 1;
    
    // Construir input
    let input = protocol::PlayerInput {
        move_forward: keyboard.pressed(KeyCode::KeyW),
        move_backward: keyboard.pressed(KeyCode::KeyS),
        move_left: keyboard.pressed(KeyCode::KeyA),
        move_right: keyboard.pressed(KeyCode::KeyD),
        jump: keyboard.pressed(KeyCode::Space),
        sprint: keyboard.pressed(KeyCode::ShiftLeft),
        camera_yaw: camera.yaw,
        camera_pitch: camera.pitch,
    };
    
    let message = ClientMessage::PlayerInput {
        sequence: sequence.0,
        input,
    };
    
    // Enviar al servidor
    if let Some(stream) = &client.stream {
        if let Ok(mut stream_lock) = stream.lock() {
            send_client_message(&mut *stream_lock, &message);
        }
    }
}

fn send_client_message(stream: &mut TcpStream, message: &ClientMessage) {
    if let Ok(json) = serde_json::to_string(message) {
        let len = json.len() as u32;
        let mut data = len.to_be_bytes().to_vec();
        data.extend(json.as_bytes());
        let _ = stream.write_all(&data);
    }
}