use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy::window::{CursorGrabMode, PrimaryWindow};
use tn1_shared::components::{Health, LocalPlayer, PlayerId};
use crate::networking::NetworkClient;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EguiPlugin)
            .init_resource::<UIState>()
            .init_resource::<ServerStats>()
            .add_systems(Update, (
                render_debug_ui,
                render_hud,
                render_instructions,
                render_server_stats,
                update_server_stats,
            ));
    }
}

#[derive(Resource, Default)]
pub struct UIState {
    pub show_debug: bool,
    pub show_inventory: bool,
    pub show_server_stats: bool,
}

#[derive(Resource, Default)]
pub struct ServerStats {
    pub connected: bool,
    pub ping: f32,
    pub players_local: u32,
    pub players_remote: u32,
    pub last_update: f32,
}

fn render_debug_ui(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UIState>,
    time: Res<Time>,
    player_query: Query<(&Transform, &PlayerId), With<LocalPlayer>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    client: Res<NetworkClient>,
) {
    let ctx = contexts.ctx_mut();

    // Toggle debug con F3
    if ctx.input(|i| i.key_pressed(egui::Key::F3)) {
        ui_state.show_debug = !ui_state.show_debug;
    }

    if !ui_state.show_debug {
        return;
    }

    egui::Window::new("🔧 Debug Info")
        .anchor(egui::Align2::LEFT_TOP, egui::vec2(10.0, 10.0))
        .resizable(false)
        .show(ctx, |ui| {
            // Performance Info
            ui.colored_label(egui::Color32::YELLOW, "📊 PERFORMANCE");
            ui.label(format!("FPS: {:.0}", 1.0 / time.delta().as_secs_f32()));
            ui.label(format!("Delta: {:.3}ms", time.delta().as_secs_f32() * 1000.0));
            ui.label(format!("Uptime: {:.1}s", time.elapsed().as_secs_f32()));
            
            ui.separator();
            
            // Network Info
            ui.colored_label(egui::Color32::LIGHT_BLUE, "🌐 NETWORKING");
            if client.connected {
                ui.colored_label(egui::Color32::GREEN, "✅ Conectado al servidor");
                ui.label("Modo: Cliente-Servidor");
                ui.label("Puerto: 127.0.0.1:7777");
            } else {
                ui.colored_label(egui::Color32::RED, "❌ Desconectado");
                ui.label("Modo: Offline (física local)");
                ui.label("Estado: Sin servidor");
            }
            
            ui.separator();
            
            // Player Info
            if let Ok((transform, player_id)) = player_query.get_single() {
                ui.colored_label(egui::Color32::GREEN, "🎮 JUGADOR");
                ui.label(format!("ID: {:?}", player_id));
                ui.label(format!("Pos X: {:.2}", transform.translation.x));
                ui.label(format!("Pos Y: {:.2}", transform.translation.y));
                ui.label(format!("Pos Z: {:.2}", transform.translation.z));
                
                ui.separator();
                ui.colored_label(egui::Color32::LIGHT_BLUE, "⚡ FÍSICA");
                
                if client.connected {
                    ui.label("Física: Servidor autoritativo");
                    // Buscar estado del jugador en los estados conocidos
                    if let Some(state) = client.player_states.get(player_id) {
                        ui.label(format!("Vel X: {:.2}", state.velocity.x));
                        ui.label(format!("Vel Y: {:.2}", state.velocity.y));
                        ui.label(format!("Vel Z: {:.2}", state.velocity.z));
                        let total_vel = state.velocity.length();
                        ui.label(format!("Vel Total: {:.2}", total_vel));
                        ui.label(format!("En suelo: {}", if state.is_grounded { "✅" } else { "❌" }));
                        ui.label(format!("Salud: {:.0}", state.health));
                    } else {
                        ui.label("Estado: Esperando datos del servidor");
                    }
                } else {
                    ui.label("Física: Sin servidor");
                    ui.label("Estado: Modo offline");
                }
                
                ui.separator();
            }
            
            // Controls Info
            ui.colored_label(egui::Color32::ORANGE, "🎯 CONTROLES ACTIVOS");
            let mut active_keys = Vec::new();
            if keyboard.pressed(KeyCode::KeyW) { active_keys.push("W"); }
            if keyboard.pressed(KeyCode::KeyA) { active_keys.push("A"); }
            if keyboard.pressed(KeyCode::KeyS) { active_keys.push("S"); }
            if keyboard.pressed(KeyCode::KeyD) { active_keys.push("D"); }
            if keyboard.pressed(KeyCode::Space) { active_keys.push("SPACE"); }
            if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) { 
                active_keys.push("SHIFT"); 
            }
            
            if active_keys.is_empty() {
                ui.label("Ninguno");
            } else {
                ui.label(format!("Activos: {}", active_keys.join(", ")));
            }
            
            ui.separator();
            ui.colored_label(egui::Color32::GRAY, "⚙️ ARQUITECTURA");
            if client.connected {
                ui.label("Modo: Cliente autoritativo del servidor");
                ui.label("Inputs: Enviados al servidor");
                ui.label("Física: Procesada en servidor");
            } else {
                ui.label("Modo: Cliente con física local");
                ui.label("Servidor: Esperando conexión");
                ui.label("Física: Temporal (local)");
            }
            
            ui.separator();
            ui.colored_label(egui::Color32::WHITE, "🔧 DEBUG COMMANDS");
            ui.label("F3: Toggle este panel");
            ui.label("F4: Print debug a consola");
            ui.label("ESC: Liberar cursor");
        });
}

fn render_hud(
    mut contexts: EguiContexts,
    player_query: Query<&Health, With<LocalPlayer>>,
) {
    let ctx = contexts.ctx_mut();

    // Crosshair simple
    egui::Area::new("crosshair".into())
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.painter().circle_filled(
                ui.cursor().center(),
                2.0,
                egui::Color32::WHITE,
            );
        });

    // Barra de salud
    if let Ok(health) = player_query.get_single() {
        egui::Area::new("health_bar".into())
            .anchor(egui::Align2::LEFT_BOTTOM, egui::vec2(20.0, -20.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("HP:");
                    let health_ratio = health.current / health.max;
                    let health_color = if health_ratio > 0.6 {
                        egui::Color32::GREEN
                    } else if health_ratio > 0.3 {
                        egui::Color32::YELLOW
                    } else {
                        egui::Color32::RED
                    };
                    
                    ui.add(egui::ProgressBar::new(health_ratio)
                        .desired_width(200.0)
                        .fill(health_color)
                        .text(format!("{:.0}/{:.0}", health.current, health.max)));
                });
            });
    }
}

fn render_instructions(
    mut contexts: EguiContexts,
    window_query: Query<&Window, With<PrimaryWindow>>,
    client: Res<NetworkClient>,
) {
    let ctx = contexts.ctx_mut();
    let Ok(window) = window_query.get_single() else { return };

    // Mostrar instrucciones solo cuando el cursor no está capturado
    if window.cursor_options.grab_mode == CursorGrabMode::None {
        egui::Window::new("🎮 Controles")
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.heading("TRUST-NO-1");
                ui.separator();
                
                if client.connected {
                    ui.colored_label(egui::Color32::GREEN, "🌐 CONECTADO AL SERVIDOR");
                    ui.label("Tu movimiento será procesado por el servidor");
                } else {
                    ui.colored_label(egui::Color32::YELLOW, "⚠️ MODO OFFLINE");
                    ui.label("Física local activa - Servidor no disponible");
                }
                
                ui.separator();
                ui.label("📱 PARA USAR LA CÁMARA CON TRACKPAD:");
                ui.label("• Haz CLIC en esta ventana para capturar el cursor");
                ui.label("• Usa el trackpad para mirar alrededor");
                ui.label("• Presiona ESC para liberar el cursor");
                
                ui.separator();
                ui.label("🎯 CONTROLES:");
                ui.label("• WASD - Movimiento");
                ui.label("• Shift - Correr (Sprint)");
                ui.label("• Espacio - Saltar");
                ui.label("• F3 - Debug info");
                ui.label("• F4 - Debug física (consola)");
                
                ui.separator();
                if client.connected {
                    ui.colored_label(egui::Color32::LIGHT_GREEN, "✅ NETWORKING ACTIVO");
                    ui.label("Inputs enviados al servidor autoritativo");
                } else {
                    ui.colored_label(egui::Color32::LIGHT_BLUE, "✅ FÍSICA LOCAL ACTIVA");
                    ui.label("El movimiento funciona localmente");
                }
                
                ui.separator();
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    if ui.button("🖱️ Capturar cursor para jugar").clicked() {
                        // El click será manejado por el sistema de cámara
                    }
                });
            });
    }
}

fn update_server_stats(
    mut server_stats: ResMut<ServerStats>,
    client: Res<NetworkClient>,
    time: Res<Time>,
    local_players: Query<(), With<LocalPlayer>>,
    remote_players: Query<(), (With<tn1_shared::components::Player>, Without<LocalPlayer>)>,
) {
    server_stats.connected = client.connected;
    server_stats.players_local = local_players.iter().count() as u32;
    server_stats.players_remote = remote_players.iter().count() as u32;
    server_stats.last_update = time.elapsed().as_secs_f32();
    
    // Ping simulado (en un juego real se mediría con timestamps)
    if client.connected {
        server_stats.ping = 15.0 + (time.elapsed().as_secs_f32().sin() * 5.0).abs();
    } else {
        server_stats.ping = 0.0;
    }
}

fn render_server_stats(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UIState>,
    server_stats: Res<ServerStats>,
    time: Res<Time>,
) {
    let ctx = contexts.ctx_mut();

    // Toggle con F1
    if ctx.input(|i| i.key_pressed(egui::Key::F1)) {
        ui_state.show_server_stats = !ui_state.show_server_stats;
    }

    if !ui_state.show_server_stats {
        return;
    }

    egui::Window::new("📊 Estado del Servidor")
        .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 10.0))
        .resizable(false)
        .show(ctx, |ui| {
            // Estado de conexión
            if server_stats.connected {
                ui.colored_label(egui::Color32::GREEN, "🟢 CONECTADO");
                ui.label(format!("📡 Ping: {:.0}ms", server_stats.ping));
            } else {
                ui.colored_label(egui::Color32::RED, "🔴 DESCONECTADO");
                ui.label("📡 Ping: ---");
            }
            
            ui.separator();
            
            // Información de jugadores
            ui.colored_label(egui::Color32::LIGHT_BLUE, "👥 JUGADORES");
            ui.label(format!("🔵 Local: {}", server_stats.players_local));
            ui.label(format!("🔴 Remotos: {}", server_stats.players_remote));
            ui.label(format!("📊 Total: {}", server_stats.players_local + server_stats.players_remote));
            
            ui.separator();
            
            // Tiempo de funcionamiento
            ui.colored_label(egui::Color32::YELLOW, "⏱️ TIEMPO");
            ui.label(format!("Uptime: {:.1}s", time.elapsed().as_secs_f32()));
            ui.label(format!("FPS: {:.0}", 1.0 / time.delta().as_secs_f32()));
            
            ui.separator();
            ui.colored_label(egui::Color32::GRAY, "💡 Presiona F1 para ocultar");
        });
}