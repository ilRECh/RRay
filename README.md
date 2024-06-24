# rray

## Description

## Reasoning

## Tech stack
- Rust
    - [ggez](https://github.com/ggez/ggez) framework
- Cargo
- Linux (Debian-based)

## Design
- [Classes](#Classes)
    - [GameState](#GameState)
    - [Player](#Player)
    - [Texture](#Texture)
    - [WorldMap](#WorldMap)
- [](#Demo)

## Classes

### GameState
It's the main class in the program, which contains input handlers and redraw functionality.

### Player
Handles the player's positioning and movements. Works with WASD inputs and supports "fishy" mouse movements.

### Texture
Contains the information about all textures:
- Walls
- Floor
- Ceiling

### WorldMap
Handles the map changes, e.g. the player's repositioning.

## Demo
https://github.com/ilRECh/rray/assets/56624748/bc8294a1-e69d-4857-8b43-26cacb4d6144

## Features
- Raycasting

## Known problems
- On the Debian-based systems the mouse capturing does not work properly. Without sudo, setting the cursor's position is impossible, and even then capturing does not work.

## How to run

```
cargo run --release
```
