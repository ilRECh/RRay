## rray

## Description

## Reasoning

## Tech stack
- Rust
    - [ggez](https://github.com/ggez/ggez) framework
- Cargo
- Linux (debian based)

## Design
- [Classes](#Classes)
    - [GameState](#GameState)
    - [Player](#Player)
    - [Texture](#Texture)
    - [WorldMap](#WorldMap)
- [](#Demo)

## Classes

### GameState
Main class in the program, which contains input handlers and redraw functionality.

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
[https://www.youtube.com/watch?v=eR_CmdlfEyQ](https://www.youtube.com/watch?v=eR_CmdlfEyQ)

## Features
- Raycasting

## Known problems
- On the Debian-based systems the mouse capturing does not work properly. Without sudo it's impossible to set the position of the cursor, and even then capturing does not work.

## How to run

```
cargo run --release
```
