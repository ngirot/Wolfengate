# Wolfengate

## Overview

This project is a basic 3D game engine inspired by Wolfenstein 3D, implemented in Rust. The engine employs raycasting to simulate a 3D environment in a 2D space, with an emphasis on efficiency and simplicity. It’s designed to provide the foundational elements needed to build a retro-style first-person shooter (FPS) with features like wall rendering, basic movement, and collision detection.
Features

- Raycasting-based 3D Rendering: The core mechanic for rendering walls and environments.
- 2D Map and Player Movement: Navigate through a 2D grid-based map from a first-person perspective.
- Textured Wall Rendering.
- Collision Detection: collision detection to prevent the player from walking through walls.
- Keyboard Input Handling: Move the player using keyboard inputs (WASD or ZQSD), and action with E.
- Basic weapons: only short range weapons yet.
- Basic enemy and HP: enemies are static.

## Building and Running the Project

Build the project: Use Cargo, the Rust package manager, to compile the project.

```
cargo build --release
```

Run the project: After building, run the binary:

```
cargo run --release
```
