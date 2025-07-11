use bevy::prelude::*;
use tn1_shared::{components::*, protocol::*};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use crate::database::Database;
use std::sync::mpsc::{self, Receiver, Sender};

// Estructura para comandos de base de datos
#[derive(Debug)]
pub enum DatabaseCommand {
    SavePlayerPosition {
        player_id: uuid::Uuid,
        position: Vec3,
        rotation: Quat,
    },
    AuthenticatePlayer {
        username: String,
        password: String,
        client_id: u32,
    },
}

#[derive(Resource)]
pub struct DatabaseChannel {
    pub sender: Sender<DatabaseCommand>,
    pub receiver: Arc<Mutex<Receiver<DatabaseCommand>>>,
}

impl DatabaseChannel {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }
}

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ServerState::new())
            .insert_resource(ServerTick(0))
            .insert_resource(SaveTimer(Timer::from_seconds(5.0, TimerMode::Repeating))) // Guardar cada 5 segundos
            .insert_resource(DatabaseChannel::new()) // Insertar el canal de base de datos
            .add_systems(Startup, (start_server, start_database_worker))
            .add_systems(Update, (
                process_client_messages,
                update_physics,
                send_world_state,
                save_player_positions, // Nuevo sistema
            ).chain());
    }
}

fn start_database_worker(
    database: Option<Res<Database>>,
    database_channel: Res<DatabaseChannel>,
) {
    if let Some(db) = database {
        let db_clone = db.clone();
        let receiver = database_channel.receiver.clone();
        
        thread::spawn(move || {
            info!("🔄 Worker de base de datos iniciado");
            
            loop {
                if let Ok(receiver_lock) = receiver.lock() {
                    match receiver_lock.recv() {
                        Ok(cmd) => {
                            // Procesar comando de base de datos
                            match cmd {
                                DatabaseCommand::SavePlayerPosition { player_id, position, rotation } => {
                                    // Crear un runtime de Tokio para la operación asíncrona
                                    let rt = tokio::runtime::Runtime::new().unwrap();
                                    match rt.block_on(db_clone.save_player_state(player_id, position, rotation)) {
                                        Ok(_) => {
                                            debug!("✅ Posición guardada para jugador: {:?}", player_id);
                                        }
                                        Err(e) => {
                                            error!("❌ Error guardando posición: {}", e);
                                        }
                                    }
                                }
                                DatabaseCommand::AuthenticatePlayer { username, password, client_id } => {
                                    let rt = tokio::runtime::Runtime::new().unwrap();
                                    match rt.block_on(db_clone.authenticate_player(&username, &password)) {
                                        Ok(Some(player)) => {
                                            info!("✅ Jugador autenticado: {} (ID: {})", username, player.id);
                                            // TODO: Enviar resultado de vuelta al sistema de networking
                                        }
                                        Ok(None) => {
                                            warn!("❌ Credenciales inválidas para: {}", username);
                                        }
                                        Err(e) => {
                                            error!("❌ Error autenticando jugador: {}", e);
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            // Canal cerrado, salir del loop
                            break;
                        }
                    }
                }
            }
            
            info!("🔄 Worker de base de datos terminado");
        });
    } else {
        warn!("⚠️ Base de datos no disponible - worker no iniciado");
    }
}

#[derive(Resource)]
pub struct ServerTick(pub u32);

#[derive(Resource)]
pub struct SaveTimer(pub Timer);

#[derive(Resource)]
pub struct ServerState {
    pub clients: Arc<Mutex<HashMap<u32, ClientConnection>>>,
    pub next_client_id: Arc<Mutex<u32>>,
    pub incoming_messages: Arc<Mutex<Vec<(u32, ClientMessage)>>>,
}

pub struct ClientConnection {
    pub stream: TcpStream,
    pub player_entity: Option<Entity>,
    pub player_id: Option<PlayerId>,
    pub player_name: String,
    pub last_ping: Instant,
}

impl ServerState {
    fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            next_client_id: Arc::new(Mutex::new(1)),
            incoming_messages: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

fn start_server(server_state: Res<ServerState>) {
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], DEFAULT_PORT)))
        .expect("Failed to bind server");
    
    println!("🌐 Servidor autoritativo iniciado en puerto {}", DEFAULT_PORT);
    println!("📊 Tick rate: {} Hz", TICK_RATE);
    
    let clients = server_state.clients.clone();
    let next_id = server_state.next_client_id.clone();
    let incoming = server_state.incoming_messages.clone();
    
    thread::spawn(move || {
        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                // Configurar stream
                stream.set_nodelay(true).ok();
                stream.set_nonblocking(false).ok();
                
                // Obtener nuevo ID
                let mut id_lock = next_id.lock().unwrap();
                let client_id = *id_lock;
                *id_lock += 1;
                drop(id_lock);
                
                println!("🔌 Nueva conexión TCP - Cliente ID: {}", client_id);
                
                // Manejar cliente en thread separado
                let clients_clone = clients.clone();
                let incoming_clone = incoming.clone();
                
                thread::spawn(move || {
                    handle_client_connection(
                        stream,
                        client_id,
                        clients_clone,
                        incoming_clone,
                    );
                });
            }
        }
    });
}

fn handle_client_connection(
    mut stream: TcpStream,
    client_id: u32,
    clients: Arc<Mutex<HashMap<u32, ClientConnection>>>,
    incoming: Arc<Mutex<Vec<(u32, ClientMessage)>>>,
) {
    let mut buffer = vec![0u8; 4096];
    let mut message_buffer = Vec::new();
    
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Cliente desconectado
                println!("📤 Cliente {} desconectado", client_id);
                incoming.lock().unwrap().push((client_id, ClientMessage::Disconnect));
                clients.lock().unwrap().remove(&client_id);
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
                        // Tenemos un mensaje completo
                        if let Ok(json_str) = std::str::from_utf8(&message_buffer[4..4 + msg_len]) {
                            if let Ok(msg) = serde_json::from_str::<ClientMessage>(json_str) {
                                // Manejar mensajes de autenticación
                                match &msg {
                                    ClientMessage::Login { protocol_version, username, .. } => {
                                        if *protocol_version != PROTOCOL_VERSION {
                                            // Versión incorrecta
                                            let error = ServerMessage::ConnectionError {
                                                reason: format!("Versión de protocolo incorrecta. Servidor: {}, Cliente: {}", 
                                                    PROTOCOL_VERSION, protocol_version)
                                            };
                                            send_message_to_stream(&mut stream, &error);
                                            break;
                                        }
                                        
                                        // Agregar cliente
                                        let mut clients_lock = clients.lock().unwrap();
                                        clients_lock.insert(client_id, ClientConnection {
                                            stream: stream.try_clone().unwrap(),
                                            player_entity: None,
                                            player_id: None,
                                            player_name: username.clone(),
                                            last_ping: Instant::now(),
                                        });
                                        drop(clients_lock);
                                        
                                        println!("✅ Cliente {} conectado: {}", client_id, username);
                                    }
                                    ClientMessage::Register { protocol_version, username, .. } => {
                                        if *protocol_version != PROTOCOL_VERSION {
                                            // Versión incorrecta
                                            let error = ServerMessage::ConnectionError {
                                                reason: format!("Versión de protocolo incorrecta. Servidor: {}, Cliente: {}", 
                                                    PROTOCOL_VERSION, protocol_version)
                                            };
                                            send_message_to_stream(&mut stream, &error);
                                            break;
                                        }
                                        
                                        // Agregar cliente
                                        let mut clients_lock = clients.lock().unwrap();
                                        clients_lock.insert(client_id, ClientConnection {
                                            stream: stream.try_clone().unwrap(),
                                            player_entity: None,
                                            player_id: None,
                                            player_name: username.clone(),
                                            last_ping: Instant::now(),
                                        });
                                        drop(clients_lock);
                                        
                                        println!("✅ Cliente {} conectado: {}", client_id, username);
                                    }
                                    ClientMessage::Reconnect { protocol_version, .. } => {
                                        if *protocol_version != PROTOCOL_VERSION {
                                            // Versión incorrecta
                                            let error = ServerMessage::ConnectionError {
                                                reason: format!("Versión de protocolo incorrecta. Servidor: {}, Cliente: {}", 
                                                    PROTOCOL_VERSION, protocol_version)
                                            };
                                            send_message_to_stream(&mut stream, &error);
                                            break;
                                        }
                                        
                                        let player_name = format!("Player_{}", client_id);
                                        
                                        // Agregar cliente
                                        let mut clients_lock = clients.lock().unwrap();
                                        clients_lock.insert(client_id, ClientConnection {
                                            stream: stream.try_clone().unwrap(),
                                            player_entity: None,
                                            player_id: None,
                                            player_name: player_name.clone(),
                                            last_ping: Instant::now(),
                                        });
                                        drop(clients_lock);
                                        
                                        println!("✅ Cliente {} reconectado: {}", client_id, player_name);
                                    }
                                    _ => {}
                                }
                                
                                // Agregar mensaje a la cola
                                incoming.lock().unwrap().push((client_id, msg));
                            }
                        }
                        
                        // Remover mensaje procesado del buffer
                        message_buffer.drain(0..4 + msg_len);
                    } else {
                        // Mensaje incompleto, esperar más datos
                        break;
                    }
                }
            }
            Err(e) => {
                if e.kind() != std::io::ErrorKind::WouldBlock {
                    println!("❌ Error leyendo de cliente {}: {}", client_id, e);
                    incoming.lock().unwrap().push((client_id, ClientMessage::Disconnect));
                    clients.lock().unwrap().remove(&client_id);
                    break;
                }
            }
        }
    }
}

fn process_client_messages(
    mut commands: Commands,
    server_state: Res<ServerState>,
    mut player_query: Query<(&mut Transform, &mut PlayerController, &PlayerId)>,
    database: Option<Res<Database>>,
    database_channel: Res<DatabaseChannel>,
) {
    let messages = {
        let mut incoming_lock = server_state.incoming_messages.lock().unwrap();
        std::mem::take(&mut *incoming_lock)
    };
    
    for (client_id, message) in messages {
        match message {
            // Temporalmente manejar los tres tipos de autenticación hasta actualizar cliente
            ClientMessage::Login { username, password, .. } => {
                handle_auth(&mut commands, &server_state, client_id, username, Some(password), None, database.as_deref(), database_channel.sender.clone());
            }
            ClientMessage::Register { username, password, .. } => {
                handle_auth(&mut commands, &server_state, client_id, username, Some(password), None, database.as_deref(), database_channel.sender.clone());
            }
            ClientMessage::Reconnect { session_token, .. } => {
                let username = format!("Player_{}", client_id);
                handle_auth(&mut commands, &server_state, client_id, username, None, Some(session_token), database.as_deref(), database_channel.sender.clone());
            }
            
            ClientMessage::PlayerInput { input, .. } => {
                // Procesar input del jugador
                let clients = server_state.clients.lock().unwrap();
                if let Some(client) = clients.get(&client_id) {
                    if let Some(entity) = client.player_entity {
                        if let Ok((mut transform, mut controller, _)) = player_query.get_mut(entity) {
                            apply_player_input(&mut transform, &mut controller, &input);
                        }
                    }
                }
            }
            
            ClientMessage::Disconnect => {
                // Remover jugador
                let mut clients = server_state.clients.lock().unwrap();
                if let Some(client) = clients.remove(&client_id) {
                    if let Some(entity) = client.player_entity {
                        commands.entity(entity).despawn();
                    }
                    
                    if let Some(player_id) = client.player_id {
                        // Notificar a otros clientes
                        let leave_msg = ServerMessage::PlayerLeft { player_id };
                        for (_, other_client) in clients.iter_mut() {
                            send_message_to_stream(&mut other_client.stream, &leave_msg);
                        }
                    }
                    
                    println!("👋 Cliente {} desconectado y limpiado", client_id);
                }
            }
            
            ClientMessage::Ping { timestamp } => {
                let clients = server_state.clients.lock().unwrap();
                if let Some(client) = clients.get(&client_id) {
                    let mut stream = client.stream.try_clone().unwrap();
                    let pong = ServerMessage::Pong { timestamp };
                    send_message_to_stream(&mut stream, &pong);
                }
            }
        }
    }
}

fn apply_player_input(
    transform: &mut Transform,
    controller: &mut PlayerController,
    input: &PlayerInput,
) {
    // Crear rotación basada en la cámara
    let yaw_rotation = Quat::from_rotation_y(input.camera_yaw);
    
    // Calcular direcciones de movimiento
    let forward = yaw_rotation * Vec3::NEG_Z;
    let right = yaw_rotation * Vec3::X;
    
    // Aplicar movimiento
    let mut movement = Vec3::ZERO;
    if input.move_forward { movement += forward; }
    if input.move_backward { movement -= forward; }
    if input.move_left { movement -= right; }
    if input.move_right { movement += right; }
    
    if movement.length() > 0.0 {
        movement = movement.normalize();
        let speed = if input.sprint { 10.0 } else { 7.0 };
        controller.velocity.x = movement.x * speed;
        controller.velocity.z = movement.z * speed;
    } else {
        controller.velocity.x *= 0.8;
        controller.velocity.z *= 0.8;
    }
    
    // Salto
    if input.jump && controller.is_grounded {
        controller.velocity.y = 8.0;
        controller.is_grounded = false;
    }
    
    // Actualizar rotación del jugador
    transform.rotation = yaw_rotation;
}

fn update_physics(
    mut player_query: Query<(&mut Transform, &mut PlayerController), With<Player>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    
    for (mut transform, mut controller) in player_query.iter_mut() {
        // Gravedad
        if !controller.is_grounded {
            controller.velocity.y -= 20.0 * dt;
        }
        
        // Aplicar velocidad
        transform.translation += controller.velocity * dt;
        
        // Colisión con suelo
        if transform.translation.y <= 0.0 {
            transform.translation.y = 0.0;
            controller.velocity.y = 0.0;
            controller.is_grounded = true;
        } else {
            controller.is_grounded = false;
        }
        
        // Límites del mundo
        transform.translation.x = transform.translation.x.clamp(-25.0, 25.0);
        transform.translation.z = transform.translation.z.clamp(-25.0, 25.0);
    }
}

fn send_world_state(
    server_state: Res<ServerState>,
    player_query: Query<(&Transform, &PlayerController, &PlayerId, &Health), With<Player>>,
    mut tick: ResMut<ServerTick>,
    time: Res<Time>,
) {
    // Incrementar tick
    tick.0 += 1;
    
    // Solo enviar cada 2 ticks (30 Hz)
    if tick.0 % 2 != 0 {
        return;
    }
    
    // Construir estado del mundo
    let mut players = Vec::new();
    for (transform, controller, player_id, health) in player_query.iter() {
        players.push(PlayerState {
            player_id: *player_id,
            position: transform.translation,
            velocity: controller.velocity,
            rotation: transform.rotation,
            health: health.current,
            is_grounded: controller.is_grounded,
            last_input_sequence: 0, // TODO: tracking de secuencias
        });
    }
    
    let world_state = ServerMessage::WorldState {
        tick: tick.0,
        players,
        timestamp: time.elapsed_secs_f64(),
    };
    
    // Enviar a todos los clientes conectados
    let clients = server_state.clients.lock().unwrap();
    for (_, client) in clients.iter() {
        if client.player_id.is_some() {
            let mut stream = client.stream.try_clone().unwrap();
            send_message_to_stream(&mut stream, &world_state);
        }
    }
}

fn send_message_to_stream(stream: &mut TcpStream, message: &ServerMessage) {
    if let Ok(json) = serde_json::to_string(message) {
        let len = json.len() as u32;
        let mut data = len.to_be_bytes().to_vec();
        data.extend(json.as_bytes());
        let _ = stream.write_all(&data);
    }
}

fn handle_auth(
    commands: &mut Commands,
    server_state: &ServerState,
    client_id: u32,
    username: String,
    password: Option<String>,
    session_token: Option<String>,
    database: Option<&Database>,
    sender: Sender<DatabaseCommand>,
) {
    let player_id = PlayerId(uuid::Uuid::new_v4());
    let spawn_pos = Vec3::new(0.0, 10.0, 0.0);
    
    // Si tenemos base de datos, intentar autenticación real
    if let Some(db) = database {
        // TODO: Implementar autenticación asíncrona
        // Por ahora continuamos con mock
        info!("🔒 Usando autenticación con base de datos para: {}", username);
    } else {
        warn!("⚠️ Autenticación mock - base de datos no disponible");
    }
    
    let entity = commands.spawn((
        Player,
        player_id,
        PlayerController::new(),
        Health::new(100.0),
        Transform::from_translation(spawn_pos),
        GlobalTransform::default(),
    )).id();
    
    // Actualizar cliente con entidad
    let mut clients = server_state.clients.lock().unwrap();
    if let Some(client) = clients.get_mut(&client_id) {
        client.player_entity = Some(entity);
        client.player_id = Some(player_id);
        client.player_name = username;
        
        // Generar token de sesión temporal
        let session_token = format!("temp_token_{}", client_id);
        
        // Enviar confirmación con nueva estructura
        let connected_msg = ServerMessage::Connected {
            player_id,
            tick_rate: TICK_RATE,
            session_token,
            spawn_position: spawn_pos,
        };
        send_message_to_stream(&mut client.stream, &connected_msg);
        
        println!("🎮 Jugador autenticado - Cliente: {}, PlayerId: {:?}", client_id, player_id);
        
        // Notificar a otros clientes
        let join_msg = ServerMessage::PlayerJoined {
            player_id,
            position: spawn_pos,
        };
        
        for (&other_id, other_client) in clients.iter_mut() {
            if other_id != client_id && other_client.player_id.is_some() {
                send_message_to_stream(&mut other_client.stream, &join_msg);
            }
        }
    }
}

fn save_player_positions(
    player_query: Query<(&Transform, &PlayerId), With<Player>>,
    database: Option<Res<Database>>,
    mut save_timer: ResMut<SaveTimer>,
    time: Res<Time>,
    database_channel: Res<DatabaseChannel>,
) {
    save_timer.0.tick(time.delta());
    
    if save_timer.0.just_finished() {
        let player_count = player_query.iter().count();
        if player_count > 0 {
            info!("💾 Guardando posiciones de {} jugadores", player_count);
            
            for (transform, player_id) in player_query.iter() {
                let cmd = DatabaseCommand::SavePlayerPosition {
                    player_id: player_id.0,
                    position: transform.translation,
                    rotation: transform.rotation,
                };
                
                if let Err(e) = database_channel.sender.send(cmd) {
                    error!("❌ Error enviando comando de guardado: {}", e);
                }
            }
        }
    }
}