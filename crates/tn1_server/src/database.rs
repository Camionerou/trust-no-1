use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use bevy::prelude::*;
use chrono::{DateTime, Utc};
use redis::aio::ConnectionManager;
use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

// Runtime de Tokio como Resource de Bevy
#[derive(Resource)]
pub struct TokioRuntime(pub tokio::runtime::Runtime);

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub database_url: String,
    pub redis_url: String,
    pub max_connections: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://tn1_admin:tn1_dev_password_2024@localhost:5432/trustno1".to_string()),
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://:tn1_redis_dev_2024@localhost:6379".to_string()),
            max_connections: 10,
        }
    }
}

#[derive(Resource, Clone)]
pub struct Database {
    pub pg_pool: PgPool,
    pub redis: Arc<Mutex<ConnectionManager>>,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_banned: bool,
    pub ban_reason: Option<String>,
    pub ban_until: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub player_id: Uuid,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub rotation_z: f32,
    pub rotation_w: f32,
    pub health: f32,
    pub hunger: f32,
    pub thirst: f32,
    pub stamina: f32,
    pub is_alive: bool,
    pub is_online: bool,
    pub last_updated: DateTime<Utc>,
}

impl Database {
    pub async fn new(config: DatabaseConfig) -> Result<Self> {
        // Conectar a PostgreSQL
        let pg_pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .connect(&config.database_url)
            .await?;

        // Conectar a Redis
        let redis_client = redis::Client::open(config.redis_url)?;
        let redis_conn = ConnectionManager::new(redis_client).await?;

        info!("✅ Conexión a base de datos establecida");

        Ok(Self {
            pg_pool,
            redis: Arc::new(Mutex::new(redis_conn)),
        })
    }

    // Autenticación
    pub async fn create_player(&self, username: &str, password: &str, email: Option<&str>) -> Result<Uuid> {
        let password_hash = hash_password(password)?;
        
        let row = sqlx::query(
            r#"
            INSERT INTO players (username, password_hash, email)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
        )
        .bind(username)
        .bind(password_hash)
        .bind(email)
        .fetch_one(&self.pg_pool)
        .await?;
        
        let player_id: Uuid = row.get("id");

        // Crear estado inicial del jugador
        sqlx::query(
            r#"
            INSERT INTO player_states (player_id)
            VALUES ($1)
            "#,
        )
        .bind(player_id)
        .execute(&self.pg_pool)
        .await?;

        // Crear perfil inicial
        sqlx::query(
            r#"
            INSERT INTO player_profiles (player_id, display_name)
            VALUES ($1, $2)
            "#,
        )
        .bind(player_id)
        .bind(username)
        .execute(&self.pg_pool)
        .await?;

        Ok(player_id)
    }

    pub async fn authenticate_player(&self, username: &str, password: &str) -> Result<Option<Player>> {
        let row = sqlx::query(
            r#"
            SELECT * FROM players
            WHERE username = $1 AND is_banned = false
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pg_pool)
        .await?;

        if let Some(row) = row {
            let player = Player {
                id: row.get("id"),
                username: row.get("username"),
                password_hash: row.get("password_hash"),
                email: row.get("email"),
                created_at: row.get("created_at"),
                last_login: row.get("last_login"),
                is_banned: row.get("is_banned"),
                ban_reason: row.get("ban_reason"),
                ban_until: row.get("ban_until"),
            };
            
            if verify_password(password, &player.password_hash)? {
                // Actualizar last_login
                sqlx::query(
                    r#"
                    UPDATE players
                    SET last_login = CURRENT_TIMESTAMP
                    WHERE id = $1
                    "#,
                )
                .bind(player.id)
                .execute(&self.pg_pool)
                .await?;

                return Ok(Some(player));
            }
        }

        Ok(None)
    }

    // Gestión de estado del jugador
    pub async fn save_player_state(&self, player_id: Uuid, position: Vec3, rotation: Quat) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE player_states
            SET position_x = $2, position_y = $3, position_z = $4,
                rotation_x = $5, rotation_y = $6, rotation_z = $7, rotation_w = $8,
                last_updated = CURRENT_TIMESTAMP
            WHERE player_id = $1
            "#,
        )
        .bind(player_id)
        .bind(position.x)
        .bind(position.y)
        .bind(position.z)
        .bind(rotation.x)
        .bind(rotation.y)
        .bind(rotation.z)
        .bind(rotation.w)
        .execute(&self.pg_pool)
        .await?;

        Ok(())
    }

    pub async fn load_player_state(&self, player_id: Uuid) -> Result<Option<PlayerState>> {
        let row = sqlx::query(
            r#"
            SELECT * FROM player_states
            WHERE player_id = $1
            "#,
        )
        .bind(player_id)
        .fetch_optional(&self.pg_pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(PlayerState {
                player_id: row.get("player_id"),
                position_x: row.get("position_x"),
                position_y: row.get("position_y"),
                position_z: row.get("position_z"),
                rotation_x: row.get("rotation_x"),
                rotation_y: row.get("rotation_y"),
                rotation_z: row.get("rotation_z"),
                rotation_w: row.get("rotation_w"),
                health: row.get("health"),
                hunger: row.get("hunger"),
                thirst: row.get("thirst"),
                stamina: row.get("stamina"),
                is_alive: row.get("is_alive"),
                is_online: row.get("is_online"),
                last_updated: row.get("last_updated"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn set_player_online(&self, player_id: Uuid, online: bool) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE player_states
            SET is_online = $2
            WHERE player_id = $1
            "#,
        )
        .bind(player_id)
        .bind(online)
        .execute(&self.pg_pool)
        .await?;

        Ok(())
    }

    // Sesiones
    pub async fn create_session(&self, player_id: Uuid, ip_address: Option<String>) -> Result<String> {
        let session_token = generate_session_token();
        
        sqlx::query(
            r#"
            INSERT INTO player_sessions (player_id, session_token, ip_address)
            VALUES ($1, $2, $3::inet)
            "#,
        )
        .bind(player_id)
        .bind(&session_token)
        .bind(ip_address)
        .execute(&self.pg_pool)
        .await?;

        // Guardar en Redis con TTL
        let mut redis = self.redis.lock().await;
        let _: () = redis::cmd("SETEX")
            .arg(format!("session:{}", session_token))
            .arg(86400) // 24 horas
            .arg(player_id.to_string())
            .query_async(&mut *redis)
            .await?;

        Ok(session_token)
    }

    pub async fn validate_session(&self, session_token: &str) -> Result<Option<Uuid>> {
        // Primero verificar en Redis (cache)
        let mut redis = self.redis.lock().await;
        let cached: Option<String> = redis::cmd("GET")
            .arg(format!("session:{}", session_token))
            .query_async(&mut *redis)
            .await?;

        if let Some(player_id_str) = cached {
            return Ok(Some(Uuid::parse_str(&player_id_str)?));
        }

        // Si no está en cache, verificar en DB
        let row = sqlx::query(
            r#"
            SELECT player_id FROM player_sessions
            WHERE session_token = $1 AND is_active = true
            "#,
        )
        .bind(session_token)
        .fetch_optional(&self.pg_pool)
        .await?;

        Ok(row.map(|r| r.get("player_id")))
    }

    pub async fn end_session(&self, session_token: &str) -> Result<()> {
        // Actualizar en DB
        sqlx::query(
            r#"
            UPDATE player_sessions
            SET ended_at = CURRENT_TIMESTAMP, is_active = false
            WHERE session_token = $1
            "#,
        )
        .bind(session_token)
        .execute(&self.pg_pool)
        .await?;

        // Eliminar de Redis
        let mut redis = self.redis.lock().await;
        let _: () = redis::cmd("DEL")
            .arg(format!("session:{}", session_token))
            .query_async(&mut *redis)
            .await?;

        Ok(())
    }
}

// Funciones auxiliares
fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Error hashing password: {}", e))?
        .to_string();
    Ok(password_hash)
}

fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| anyhow::anyhow!("Invalid password hash: {}", e))?;
    let argon2 = Argon2::default();
    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

fn generate_session_token() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    let mut rng = rand::thread_rng();
    (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub struct DatabasePlugin;

impl Plugin for DatabasePlugin {
    fn build(&self, app: &mut App) {
        // Creamos un runtime de Tokio que vivirá durante toda la aplicación
        let runtime = tokio::runtime::Runtime::new().unwrap();
        
        // Inicializamos la base de datos de forma bloqueante al inicio
        let database = runtime.block_on(async {
            match Database::new(DatabaseConfig::default()).await {
                Ok(db) => {
                    info!("✅ Base de datos inicializada correctamente");
                    Some(db)
                }
                Err(e) => {
                    error!("❌ Error inicializando base de datos: {}", e);
                    error!("❗ El servidor continuará sin persistencia");
                    None
                }
            }
        });
        
        // Si la DB se inicializó correctamente, la añadimos como recurso
        if let Some(db) = database {
            app.insert_resource(db);
            app.insert_resource(TokioRuntime(runtime));
        }
    }
}