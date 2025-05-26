# 🦀 Rust Colony Simulation

An ecosim written in rust.

![License](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)

## Clone and build demo (:
https://youtu.be/IlihsOJ06CY

## Overview

This simulation is pretty simple. There are: Gatherers that seek resources, predators that hunt gatherers, and resources that regenerate over time.

## Features

- Real-time simulation
- Three distinct entity types with unique behaviors
- An ecosystem with energy-based interactions
- Automatic population management and resource spawning
- Interactive controls for manual entity spawning

## Quick Start

### Prerequisites
- Rust (latest stable) - [Install here](https://rustup.rs/)
- Graphics-capable system (Windows/macOS/Linux)

### Running the Simulation

```bash
git clone git@github.com:Ether-G/rustcolony.git
cd rustcolony
cargo run
```

### Controls
- **ESC**: Exit simulation
- **SPACE**: Add 5 random resources
- **G**: Add 3 gatherers
- **P**: Add 1 predator

## Entity Types

### Gatherers (Green)
- Seek and consume resources for energy
- Move randomly when not targeting resources
- Display energy bars above entities
- Die when energy reaches zero

### Resources (Yellow)
- Static entities that regenerate energy over time
- Provide energy to gatherers when consumed
- Display pulsing visual effects when at full energy
- Automatically respawn to maintain ecosystem balance

### Predators (Red)
- Hunt gatherers for energy
- More aggressive movement patterns
- Display spike decorations around entity
- Consume energy faster than gatherers

## Technical Implementation

### Architecture
- See Mermaid Diagram in Repo Head (:

### Dependencies
- **minifb**: Framebuffer graphics library for cross-platform rendering
- **rand**: Random number generation for entity behaviors and spawning

### Performance
- Target: 60 FPS with 100+ entities
- Optimized entity interactions and rendering pipeline
- Efficient spatial queries for entity targeting

## Project Structure

```
src/
├── main.rs          # Application entry point and main loop
├── simulation.rs    # Core simulation logic and entity management
├── entity.rs        # Entity definitions and behaviors
├── renderer.rs      # Graphics rendering system
└── position.rs      # 2D position utilities

Cargo.toml           # Project dependencies and metadata
README.md            # Project documentation
```

## Ecosystem Dynamics

The simulation maintains ecological balance through:
- Energy-based entity lifecycles
- Automatic population management
- Resource regeneration systems
- Predator-prey relationships

Entities exhibit emergent behaviors through:
- Resource-seeking algorithms for gatherers
- Hunting behaviors for predators
- Adaptive movement based on energy levels
- Spatial awareness and collision detection

## Configuration

Entity parameters can be modified in `src/entity.rs`:
- Starting energy levels
- Movement speeds
- Interaction ranges
- Energy consumption rates

Simulation parameters in `src/simulation.rs`:
- Spawn rates and population limits
- World dimensions
- Update frequencies

## Contributing

Contributions are welcome. Areas for enhancement:
- Additional entity types and behaviors
- Spatial partitioning for performance optimization
- Advanced AI behaviors and pathfinding
- Save/load functionality
- Network multiplayer support

## License
Take my code and do whatever ya want with it (:

MIT License - See LICENSE file for details. 
