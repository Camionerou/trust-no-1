-- TRUST-NO-1 Database Schema
-- Version: Alpha 0.0.2

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Players table: Core player data
CREATE TABLE IF NOT EXISTS players (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(32) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP WITH TIME ZONE,
    is_banned BOOLEAN DEFAULT FALSE,
    ban_reason TEXT,
    ban_until TIMESTAMP WITH TIME ZONE
);

-- Player profiles: Extended player information
CREATE TABLE IF NOT EXISTS player_profiles (
    player_id UUID PRIMARY KEY REFERENCES players(id) ON DELETE CASCADE,
    display_name VARCHAR(32),
    total_playtime INTEGER DEFAULT 0, -- seconds
    kills INTEGER DEFAULT 0,
    deaths INTEGER DEFAULT 0,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Player states: Current game state (position, health, etc)
CREATE TABLE IF NOT EXISTS player_states (
    player_id UUID PRIMARY KEY REFERENCES players(id) ON DELETE CASCADE,
    -- Position
    position_x REAL NOT NULL DEFAULT 0.0,
    position_y REAL NOT NULL DEFAULT 10.0,
    position_z REAL NOT NULL DEFAULT 0.0,
    -- Rotation (quaternion)
    rotation_x REAL NOT NULL DEFAULT 0.0,
    rotation_y REAL NOT NULL DEFAULT 0.0,
    rotation_z REAL NOT NULL DEFAULT 0.0,
    rotation_w REAL NOT NULL DEFAULT 1.0,
    -- Vital stats
    health REAL NOT NULL DEFAULT 100.0,
    hunger REAL NOT NULL DEFAULT 100.0,
    thirst REAL NOT NULL DEFAULT 100.0,
    stamina REAL NOT NULL DEFAULT 100.0,
    -- State
    is_alive BOOLEAN DEFAULT TRUE,
    is_online BOOLEAN DEFAULT FALSE,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Sessions: Track player sessions for analytics and security
CREATE TABLE IF NOT EXISTS player_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    session_token VARCHAR(255) UNIQUE NOT NULL,
    ip_address INET,
    started_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    ended_at TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN DEFAULT TRUE
);

-- Indexes for performance
CREATE INDEX idx_players_username ON players(username);
CREATE INDEX idx_players_email ON players(email);
CREATE INDEX idx_player_sessions_token ON player_sessions(session_token);
CREATE INDEX idx_player_sessions_active ON player_sessions(player_id, is_active);
CREATE INDEX idx_player_states_online ON player_states(is_online);

-- Function to update timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers
CREATE TRIGGER update_player_profiles_updated_at BEFORE UPDATE
    ON player_profiles FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_player_states_updated_at BEFORE UPDATE
    ON player_states FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Initial data for testing (optional)
-- INSERT INTO players (username, password_hash) VALUES 
-- ('test_player', crypt('password123', gen_salt('bf')));