# BALLISTICS SYSTEM - TRUST-NO-1

## Visión General

El sistema balístico de TRUST-NO-1 simula física realista de proyectiles, incluyendo gravedad, resistencia del aire, penetración de materiales y fragmentación, creando un combate táctico y predecible.

## Física de Proyectiles

### Factores que Afectan la Trayectoria

#### Gravedad
- Aceleración constante: 9.81 m/s²
- Caída del proyectil sobre distancia
- Mayor efecto en proyectiles lentos
- Cálculo parabólico

#### Resistencia del Aire
- Proporcional al cuadrado de velocidad
- Coeficiente según forma del proyectil
- Reduce velocidad sobre distancia
- Mayor efecto a largas distancias

#### Viento
- Dirección y fuerza variables
- Afecta más a proyectiles ligeros
- Deflexión lateral
- Sistema de clima dinámico

#### Efecto Coriolis
- Solo a distancias extremas (1000m+)
- Basado en latitud del mapa
- Deflexión mínima pero presente

### Tipos de Munición

#### Full Metal Jacket (FMJ)
```
Penetración: Alta
Daño: Medio
Fragmentación: Baja
Velocidad: Alta
Uso: General purpose
```

#### Hollow Point (HP)
```
Penetración: Baja
Daño: Alto (sin armor)
Fragmentación: Alta
Velocidad: Media
Uso: Anti-personal
```

#### Armor Piercing (AP)
```
Penetración: Muy Alta
Daño: Medio-Bajo
Fragmentación: Muy Baja
Velocidad: Alta
Uso: Anti-armor
```

#### Armor Piercing Incendiary (API)
```
Penetración: Alta
Daño: Alto
Fragmentación: Media
Efectos: Fuego
Uso: Anti-vehículo
```

#### Subsónica
```
Penetración: Baja
Daño: Medio
Fragmentación: Baja
Velocidad: <343 m/s
Uso: Stealth
```

### Calibres y Características

#### Pistolas
- 9x19mm: Versátil, bajo retroceso
- .45 ACP: Alto stopping power
- 5.7x28mm: Alta penetración
- .357: Daño alto, retroceso alto

#### Rifles de Asalto
- 5.56x45mm: Precisión, baja caída
- 7.62x39mm: Daño alto, más caída
- 5.45x39mm: Balance penetración/daño

#### Rifles de Francotirador
- 7.62x51mm: Largo alcance estándar
- .338 Lapua: Extremo largo alcance
- 12.7x99mm: Anti-material

#### Escopetas
- Buckshot: Múltiples proyectiles
- Slug: Proyectil único pesado
- Flechette: Penetración alta
- Dragon's Breath: Incendiario

## Sistema de Penetración

### Mecánica de Penetración

#### Cálculo de Penetración
1. Energía del proyectil al impacto
2. Resistencia del material
3. Ángulo de impacto
4. Grosor del material

#### Materiales y Resistencia
```
Carne:      Resistencia 1
Madera:     Resistencia 3
Concreto:   Resistencia 8
Metal:      Resistencia 15
Armor:      Variable (10-50)
```

#### Post-Penetración
- Reducción de velocidad
- Cambio de trayectoria
- Posible fragmentación
- Daño reducido

### Ricochets

#### Condiciones
- Ángulo menor a 30°
- Material duro
- Proyectil intacto
- Velocidad suficiente

#### Comportamiento
- Nueva trayectoria calculada
- Pérdida de velocidad (30-50%)
- Posible deformación
- Daño reducido

## Sistema de Daño

### Multiplicadores por Zona

#### Zonas Críticas
```
Cabeza:         2.5x - 3.0x (letal con mayoría)
Cuello:         2.0x
Corazón:        1.8x
Pulmones:       1.5x
```

#### Zonas Estándar
```
Torso:          1.0x
Brazos:         0.7x
Piernas:        0.7x
Manos/Pies:     0.5x
```

### Tipos de Daño

#### Daño Directo
- Impacto del proyectil
- Basado en energía cinética
- Modificado por tipo de munición

#### Daño por Fragmentación
- Probabilidad según munición
- Múltiples fragmentos
- Daño en área
- Sangrado adicional

#### Daño por Cavitación
- Cavity temporal
- Daño a órganos cercanos
- Mayor con alta velocidad

### Efectos Especiales

#### Sangrado
- Severidad según calibre
- Localización de herida
- Requiere tratamiento médico

#### Shock
- Trauma por impacto
- Visión borrosa temporal
- Reducción de stamina

#### Supresión
- Balas cercanas causan efecto
- Reducción de precisión
- Aumento de sway
- Efecto psicológico

## Armaduras y Protección

### Niveles de Protección (NIJ)

#### Level IIA
- Detiene: 9mm, .40 S&W
- Peso: Ligero
- Movilidad: 95%

#### Level II
- Detiene: 9mm +P, .357
- Peso: Ligero-Medio
- Movilidad: 90%

#### Level IIIA
- Detiene: .44 Magnum, SMG rounds
- Peso: Medio
- Movilidad: 85%

#### Level III
- Detiene: Rifles hasta 7.62x51
- Peso: Pesado
- Movilidad: 75%

#### Level IV
- Detiene: AP rounds, .30-06
- Peso: Muy Pesado
- Movilidad: 65%

### Degradación de Armor

#### Factores
- Impactos recibidos
- Calibre del impacto
- Tipo de munición
- Área impactada

#### Estados
```
Nuevo:          100% efectividad
Usado:          80% efectividad
Dañado:         60% efectividad
Crítico:        30% efectividad
Destruido:      0% efectividad
```

## Armas y Modificadores

### Estadísticas Base

#### Precisión
- MOA (Minute of Angle)
- Afectada por calidad del arma
- Modificadores por attachments
- Degradación por uso

#### Velocidad de Salida
- m/s según calibre y barrel
- Afecta trayectoria
- Modificada por barrel length
- Supresores reducen velocidad

#### Cadencia de Fuego
- RPM (Rounds per minute)
- Semi-auto vs Full-auto
- Burst fire modes
- Heat buildup

### Modificadores por Attachments

#### Muzzle Devices
```
Supresor:       -Ruido, -Flash, -Velocidad
Compensador:    -Recoil vertical
Freno:          -Recoil general, +Ruido
Flash Hider:    -Flash visible
```

#### Barrels
```
Corto:          +Movilidad, -Precisión, -Alcance
Estándar:       Balanceado
Largo:          +Precisión, +Alcance, -Movilidad
Heavy:          +Precisión, -Recoil, --Movilidad
```

#### Optics
```
Iron Sights:    Rápido, corto alcance
Red Dot:        Rápido, versátil
Holographic:    Campo de visión amplio
ACOG:           Media distancia
Sniper Scope:   Larga distancia, -FOV
```

## Factores Ambientales

### Clima

#### Lluvia
- Reduce visibilidad
- Afecta trayectoria mínimamente
- Aumenta ruido ambiental
- Degrada armas más rápido

#### Niebla
- Reduce alcance efectivo
- Dificulta identificación
- Ventaja para stealth

#### Temperatura
- Afecta densidad del aire
- Modifica resistencia
- Expansión/contracción de materiales

### Altitud
- Menor densidad de aire
- Mayor alcance efectivo
- Menos caída de bala
- Ajuste de miras necesario

## Sistema de Detección

### Sonido de Disparos

#### Propagación
- Velocidad del sonido: 343 m/s
- Atenuación por distancia
- Reflexión en superficies
- Oclusión por obstáculos

#### Supresión
- Reduce alcance de detección 70%
- No elimina sonido completamente
- Subsónicas más silenciosas

### Flash y Humo
- Visible según condiciones
- Delata posición
- Persistencia de humo
- Trazadoras opcionales

## Validación Server-Side

### Anti-Cheat Balístico

#### Verificaciones
- Trayectoria físicamente posible
- Line of sight válido
- Timing consistente
- Energía de impacto correcta

#### Límites
- Alcance máximo por arma
- Penetración máxima realista
- Daño dentro de rangos
- No-clip detection

## Performance

### Optimizaciones

#### Projectile Pooling
- Reutilizar objetos proyectil
- Límite máximo simultáneo
- Cleanup automático

#### LOD para Trayectorias
- Alta precisión cerca
- Aproximación a distancia
- Skip frames en vuelo largo

#### Batch Processing
- Múltiples proyectiles juntos
- Spatial partitioning
- Culling por relevancia

---

Este sistema balístico proporciona combate realista y predecible, recompensando conocimiento y habilidad.