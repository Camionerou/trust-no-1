# DATABASE ARCHITECTURE - TRUST-NO-1

Arquitectura completa de base de datos para soportar un mundo persistente con miles de jugadores simultáneos.

## Visión General

El sistema utiliza:
- **PostgreSQL 15+**: Almacenamiento principal persistente
- **Redis 7+**: Cache, sesiones y estado temporal
- **TimescaleDB**: Extensión para datos de series temporales (logs, métricas)

## PostgreSQL - Esquema Principal

### Cuentas y Jugadores

#### accounts
Información de cuenta del jugador (login, seguridad)
```sql
- id: UUID PRIMARY KEY
- username: VARCHAR(32) UNIQUE NOT NULL
- email: VARCHAR(255) UNIQUE NOT NULL  
- password_hash: VARCHAR(255) NOT NULL
- created_at: TIMESTAMPTZ DEFAULT NOW()
- last_login: TIMESTAMPTZ
- banned: BOOLEAN DEFAULT FALSE
- ban_reason: TEXT
- ban_expires: TIMESTAMPTZ
- two_factor_enabled: BOOLEAN DEFAULT FALSE
- premium_expires: TIMESTAMPTZ
```

#### players
Personajes de los jugadores en el mundo
```sql
- id: UUID PRIMARY KEY
- account_id: UUID REFERENCES accounts(id)
- character_name: VARCHAR(32) UNIQUE NOT NULL
- position: JSONB NOT NULL -- {x, y, z}
- rotation: JSONB NOT NULL -- {x, y, z, w}
- chunk_x: INTEGER NOT NULL
- chunk_z: INTEGER NOT NULL
- health: JSONB NOT NULL -- Sistema completo de salud
- stats: JSONB NOT NULL -- Estadísticas y skills
- faction_id: UUID REFERENCES factions(id)
- playtime_seconds: BIGINT DEFAULT 0
- created_at: TIMESTAMPTZ DEFAULT NOW()
- last_seen: TIMESTAMPTZ DEFAULT NOW()
- is_online: BOOLEAN DEFAULT FALSE
```

#### player_stats
Estadísticas detalladas del jugador
```sql
- player_id: UUID PRIMARY KEY REFERENCES players(id)
- kills_players: INTEGER DEFAULT 0
- kills_npcs: INTEGER DEFAULT 0
- deaths: INTEGER DEFAULT 0
- distance_traveled: FLOAT DEFAULT 0
- shots_fired: INTEGER DEFAULT 0
- shots_hit: INTEGER DEFAULT 0
- damage_dealt: FLOAT DEFAULT 0
- damage_received: FLOAT DEFAULT 0
- items_crafted: INTEGER DEFAULT 0
- buildings_placed: INTEGER DEFAULT 0
- resources_gathered: JSONB DEFAULT '{}'
```

### Sistema de Inventarios

#### inventories
Contenedores de items (jugadores, cofres, vehículos)
```sql
- id: UUID PRIMARY KEY
- owner_id: UUID NOT NULL
- owner_type: inventory_owner_type NOT NULL
- grid_width: SMALLINT NOT NULL
- grid_height: SMALLINT NOT NULL
- max_weight: FLOAT
- locked: BOOLEAN DEFAULT FALSE
- lock_code: VARCHAR(10)
- created_at: TIMESTAMPTZ DEFAULT NOW()
```

#### inventory_items  
Items dentro de inventarios
```sql
- id: UUID PRIMARY KEY
- inventory_id: UUID REFERENCES inventories(id) ON DELETE CASCADE
- item_template_id: VARCHAR(64) NOT NULL
- grid_x: SMALLINT NOT NULL
- grid_y: SMALLINT NOT NULL
- grid_width: SMALLINT NOT NULL
- grid_height: SMALLINT NOT NULL
- quantity: INTEGER DEFAULT 1
- durability: FLOAT
- attachments: JSONB -- Para armas modulares
- custom_data: JSONB -- Datos específicos del item
- created_at: TIMESTAMPTZ DEFAULT NOW()
- UNIQUE(inventory_id, grid_x, grid_y) -- Prevenir solapamiento
```

### Mundo y Estructuras

#### world_chunks
División del mundo en chunks para carga eficiente
```sql
- x: INTEGER NOT NULL
- z: INTEGER NOT NULL  
- biome_type: VARCHAR(32) NOT NULL
- terrain_data: BYTEA -- Heightmap comprimido
- resource_nodes: JSONB -- Spawns de recursos
- last_loaded: TIMESTAMPTZ
- last_modified: TIMESTAMPTZ DEFAULT NOW()
- PRIMARY KEY (x, z)
```

#### structures
Construcciones de jugadores
```sql
- id: UUID PRIMARY KEY
- chunk_x: INTEGER NOT NULL
- chunk_z: INTEGER NOT NULL
- position: JSONB NOT NULL -- {x, y, z}
- rotation: JSONB NOT NULL
- structure_type: VARCHAR(64) NOT NULL
- owner_id: UUID REFERENCES players(id)
- faction_id: UUID REFERENCES factions(id)
- health: FLOAT NOT NULL
- max_health: FLOAT NOT NULL
- build_data: JSONB NOT NULL -- Componentes modulares
- permissions: JSONB NOT NULL -- Acceso por jugador/clan
- upkeep_paid_until: TIMESTAMPTZ
- decay_protection: BOOLEAN DEFAULT TRUE
- created_at: TIMESTAMPTZ DEFAULT NOW()
- last_damaged: TIMESTAMPTZ
- INDEX idx_structures_chunk (chunk_x, chunk_z)
- INDEX idx_structures_owner (owner_id)
```

#### structure_storage
Almacenamiento asociado a estructuras
```sql
- structure_id: UUID PRIMARY KEY REFERENCES structures(id)
- inventory_id: UUID REFERENCES inventories(id)
- authorized_players: UUID[] -- Array de player IDs
- tc_range: FLOAT DEFAULT 30.0 -- Tool cupboard range
```

### Vehículos

#### vehicles
Todos los vehículos del mundo
```sql
- id: UUID PRIMARY KEY
- template_id: VARCHAR(64) NOT NULL
- position: JSONB NOT NULL
- rotation: JSONB NOT NULL
- chunk_x: INTEGER NOT NULL
- chunk_z: INTEGER NOT NULL
- owner_id: UUID REFERENCES players(id)
- faction_id: UUID REFERENCES factions(id)
- fuel_current: FLOAT NOT NULL
- fuel_max: FLOAT NOT NULL
- health: FLOAT NOT NULL
- max_health: FLOAT NOT NULL
- engine_damage: FLOAT DEFAULT 0
- tire_damage: JSONB -- Por rueda
- modifications: JSONB -- Upgrades instalados
- inventory_id: UUID REFERENCES inventories(id)
- locked: BOOLEAN DEFAULT FALSE
- key_code: VARCHAR(10)
- last_used: TIMESTAMPTZ DEFAULT NOW()
- created_at: TIMESTAMPTZ DEFAULT NOW()
```

### Clanes y Territorios

#### factions
Clanes/facciones de jugadores
```sql
- id: UUID PRIMARY KEY
- name: VARCHAR(64) UNIQUE NOT NULL
- tag: VARCHAR(8) UNIQUE NOT NULL
- description: TEXT
- leader_id: UUID REFERENCES players(id)
- treasury: BIGINT DEFAULT 0
- reputation: INTEGER DEFAULT 0
- color: CHAR(7) -- Hex color
- discord_webhook: TEXT
- created_at: TIMESTAMPTZ DEFAULT NOW()
- member_count: INTEGER DEFAULT 1
- max_members: INTEGER DEFAULT 50
```

#### faction_members
Miembros y roles en facciones
```sql
- faction_id: UUID REFERENCES factions(id) ON DELETE CASCADE
- player_id: UUID REFERENCES players(id) ON DELETE CASCADE
- rank: faction_rank NOT NULL
- permissions: JSONB NOT NULL
- joined_at: TIMESTAMPTZ DEFAULT NOW()
- last_online: TIMESTAMPTZ DEFAULT NOW()
- contribution_points: INTEGER DEFAULT 0
- PRIMARY KEY (faction_id, player_id)
```

#### territories
Zonas controlables del mapa
```sql
- id: UUID PRIMARY KEY
- name: VARCHAR(64) NOT NULL
- center_x: FLOAT NOT NULL
- center_z: FLOAT NOT NULL
- radius: FLOAT NOT NULL
- owner_faction_id: UUID REFERENCES factions(id)
- captured_at: TIMESTAMPTZ
- capture_points: INTEGER DEFAULT 0
- max_capture_points: INTEGER DEFAULT 100
- resources_generated: JSONB -- Recursos por hora
- under_attack: BOOLEAN DEFAULT FALSE
- last_attacked: TIMESTAMPTZ
- building_allowed: BOOLEAN DEFAULT TRUE
```

### Economía y Comercio

#### market_listings
Mercado global de jugadores
```sql
- id: UUID PRIMARY KEY
- seller_id: UUID REFERENCES players(id)
- item_template_id: VARCHAR(64) NOT NULL
- quantity: INTEGER NOT NULL
- price_per_unit: BIGINT NOT NULL
- currency: currency_type DEFAULT 'scrap'
- quality: FLOAT -- Para items con durabilidad
- attachments: JSONB -- Para armas
- location_type: market_location_type NOT NULL
- location_id: UUID -- Referencia a vending machine o mercado
- expires_at: TIMESTAMPTZ NOT NULL
- created_at: TIMESTAMPTZ DEFAULT NOW()
- sold_quantity: INTEGER DEFAULT 0
- INDEX idx_market_item (item_template_id)
- INDEX idx_market_expires (expires_at)
```

#### transactions
Historial de transacciones
```sql
- id: UUID PRIMARY KEY
- buyer_id: UUID REFERENCES players(id)
- seller_id: UUID REFERENCES players(id)
- listing_id: UUID REFERENCES market_listings(id)
- item_template_id: VARCHAR(64) NOT NULL
- quantity: INTEGER NOT NULL
- total_price: BIGINT NOT NULL
- currency: currency_type DEFAULT 'scrap'
- transaction_type: transaction_type NOT NULL
- location: JSONB
- created_at: TIMESTAMPTZ DEFAULT NOW()
```

### Eventos y Logs

#### world_events
Eventos dinámicos del mundo
```sql
- id: UUID PRIMARY KEY
- event_type: VARCHAR(64) NOT NULL
- name: VARCHAR(128)
- description: TEXT
- location: JSONB NOT NULL -- {x, y, z, radius}
- chunk_x: INTEGER NOT NULL
- chunk_z: INTEGER NOT NULL
- participants: UUID[] -- Player IDs
- rewards: JSONB
- data: JSONB NOT NULL -- Datos específicos del evento
- started_at: TIMESTAMPTZ DEFAULT NOW()
- ends_at: TIMESTAMPTZ NOT NULL
- completed: BOOLEAN DEFAULT FALSE
- winner_id: UUID REFERENCES players(id)
```

#### combat_logs
Logs de combate para análisis
```sql
- id: UUID PRIMARY KEY
- attacker_id: UUID REFERENCES players(id)
- victim_id: UUID REFERENCES players(id)
- weapon_id: VARCHAR(64)
- ammo_type: VARCHAR(64)
- damage: FLOAT NOT NULL
- hit_location: body_part NOT NULL
- distance: FLOAT
- headshot: BOOLEAN DEFAULT FALSE
- killing_blow: BOOLEAN DEFAULT FALSE
- position_attacker: JSONB
- position_victim: JSONB
- created_at: TIMESTAMPTZ DEFAULT NOW()
- INDEX idx_combat_time (created_at DESC)
- INDEX idx_combat_players (attacker_id, victim_id)
```

#### raid_logs
Registro de raids a bases
```sql
- id: UUID PRIMARY KEY
- structure_id: UUID REFERENCES structures(id)
- raider_faction_id: UUID REFERENCES factions(id)
- defender_faction_id: UUID REFERENCES factions(id)
- started_at: TIMESTAMPTZ DEFAULT NOW()
- ended_at: TIMESTAMPTZ
- successful: BOOLEAN
- damage_dealt: FLOAT
- explosives_used: JSONB
- loot_taken: JSONB
- participants: JSONB -- {raiders: [], defenders: []}
```

### Tipos ENUM

```sql
CREATE TYPE inventory_owner_type AS ENUM ('player', 'structure', 'vehicle', 'container', 'corpse');
CREATE TYPE faction_rank AS ENUM ('leader', 'officer', 'member', 'recruit');
CREATE TYPE body_part AS ENUM ('head', 'thorax', 'stomach', 'left_arm', 'right_arm', 'left_leg', 'right_leg');
CREATE TYPE currency_type AS ENUM ('scrap', 'hqm', 'credits');
CREATE TYPE market_location_type AS ENUM ('global', 'vending_machine', 'player_shop', 'safe_zone');
CREATE TYPE transaction_type AS ENUM ('market_purchase', 'direct_trade', 'faction_bank', 'quest_reward');
```

### Índices Críticos

```sql
-- Performance crítico
CREATE INDEX idx_players_online ON players(is_online) WHERE is_online = true;
CREATE INDEX idx_players_chunk ON players(chunk_x, chunk_z) WHERE is_online = true;
CREATE INDEX idx_inventory_items_template ON inventory_items(item_template_id);
CREATE INDEX idx_structures_upkeep ON structures(upkeep_paid_until) WHERE decay_protection = true;
CREATE INDEX idx_world_events_active ON world_events(ends_at) WHERE completed = false;

-- Búsquedas frecuentes
CREATE INDEX idx_players_faction ON players(faction_id);
CREATE INDEX idx_market_search ON market_listings(item_template_id, price_per_unit) WHERE expires_at > NOW();
CREATE INDEX idx_combat_recent ON combat_logs(created_at DESC);
```

## Redis - Cache y Estado Temporal

### Estructura de Keys

#### Estado del Jugador
```
player:{id}:state           # Estado actual (health, position, etc)
player:{id}:combat          # Estado de combate (in_combat, last_damage)
player:{id}:buffs           # Efectos activos
player:{id}:cooldowns       # Cooldowns de habilidades
player:{id}:session         # Token de sesión actual
```

#### Chunks Activos
```
chunk:{x}:{z}:players       # SET de player IDs en el chunk
chunk:{x}:{z}:entities      # Hash de entidades (items, NPCs)
chunk:{x}:{z}:events        # Eventos activos en el chunk
chunk:{x}:{z}:modified      # Timestamp de última modificación
```

#### Sistema de Combate
```
damage:queue                # LIST - Cola de daño a procesar
projectiles:active          # HASH - Proyectiles en vuelo
combat:recent:{player_id}   # ZSET - Combates recientes (anti combat-log)
```

#### Mercado
```
market:prices:{item_id}     # Precio promedio (sliding window)
market:volume:{item_id}     # Volumen de trading
market:trends               # HASH - Tendencias de precios
```

#### Colas de Procesamiento
```
queue:loot_generation       # LIST - Generar loot
queue:structure_decay       # LIST - Procesar decay
queue:event_spawn          # LIST - Spawn de eventos
queue:ai_pathfinding       # LIST - Cálculos de IA
```

#### Leaderboards
```
leaderboard:kills:daily     # ZSET - Top asesinos del día
leaderboard:kills:weekly    # ZSET - Top asesinos de la semana
leaderboard:wealth          # ZSET - Jugadores más ricos
leaderboard:playtime        # ZSET - Más tiempo jugado
leaderboard:faction:power   # ZSET - Facciones más poderosas
```

#### Sesiones y Matchmaking
```
sessions:active             # SET - Todas las sesiones activas
matchmaking:queue:{region}  # LIST - Cola de matchmaking por región
server:status:{id}          # HASH - Estado del servidor
```

### TTL y Expiración

- Estado del jugador: 5 minutos (renovar mientras online)
- Chunks inactivos: 10 minutos
- Sesiones: 24 horas
- Logs de combate: 1 hora
- Leaderboards: Persistente (actualizar cada hora)
- Colas: Sin expiración (procesar FIFO)

## Optimizaciones

### Particionamiento
- Particionar `combat_logs` por mes
- Particionar `transactions` por año
- Particionar `world_events` por estado (activo/completado)

### Archivado
- Mover logs antiguos a almacenamiento frío después de 3 meses
- Comprimir datos de chunks inactivos
- Archivar cuentas inactivas después de 1 año

### Replicación
- Master-slave para PostgreSQL
- Redis Cluster para alta disponibilidad
- Backups incrementales cada hora
- Snapshots completos diarios

### Monitoreo
- Alertas en queries lentas (>100ms)
- Monitoreo de tamaño de tablas
- Análisis de índices no utilizados
- Vacuum automático agresivo

## Procedimientos Almacenados

### Transferencia de Items
Procedimiento para mover items entre inventarios con validación

### Cálculo de Decay
Procedimiento para calcular y aplicar decay a estructuras

### Procesamiento de Combate
Procedimiento para aplicar daño con todas las validaciones

### Actualización de Economía
Procedimiento para actualizar precios de mercado

## Backup y Recuperación

### Estrategia de Backup
1. **Continuo**: WAL archiving para PostgreSQL
2. **Snapshots**: Cada 6 horas para recuperación rápida
3. **Completo**: Backup completo diario
4. **Offsite**: Replicación a otra región

### Plan de Recuperación
1. **RPO** (Recovery Point Objective): 5 minutos
2. **RTO** (Recovery Time Objective): 30 minutos
3. **Pruebas**: Simulacro mensual de recuperación
4. **Documentación**: Procedimientos paso a paso

## Consideraciones de Escalabilidad

### Sharding Horizontal
- Preparado para sharding por región geográfica
- Player IDs incluyen shard identifier
- Cross-shard trading limitado

### Crecimiento Esperado
- 10,000 jugadores concurrentes: Configuración actual
- 50,000 jugadores: Agregar read replicas
- 100,000+ jugadores: Implementar sharding

### Límites de Diseño
- Máximo 1000 jugadores por servidor
- Máximo 10,000 estructuras por chunk
- Máximo 1M items activos por servidor

---

Esta arquitectura está diseñada para escalar horizontalmente y manejar la complejidad de un mundo persistente masivo.