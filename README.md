# Wolfengate

## Overview

This project is a basic 3D game engine inspired by Wolfenstein 3D, implemented in Rust. The engine employs raycasting to simulate a 3D environment in a 2D space, with an emphasis on efficiency and simplicity. It’s designed to provide the foundational elements needed to build a retro-style first-person shooter (FPS) with features like wall rendering, basic movement, and collision detection.

You can take a look at the engine in action in this short video:

[![IMAGE ALT TEXT HERE](https://img.youtube.com/vi/J3RBloS4hno/0.jpg)](https://www.youtube.com/watch?v=J3RBloS4hno)

Main features:
- Raycasting-based 3D Rendering: The core mechanic for rendering walls and environments.
- 2D Map and Player Movement: Navigate through a 2D grid-based map from a first-person perspective.
- Textured Wall Rendering.
- Collision Detection: collision detection to prevent the player from walking through walls.
- Keyboard Input Handling: Move the player using keyboard inputs (WASD or ZQSD), and action with E.
- Basic weapons: only short range weapons yet.
- Basic enemy and HP: enemies are static.
- Transparency tiles

## Configuration

### Map
Map is in an ascii format, and each char is a tile, object or ennemy on the level.
```
##############
#      #     #
#  #   #######
#  d     #  E#
#  #d##  # ###
#     #      #
#### ####G#D##
#            #
#            #
#        #   #
#    E   D   #
#        #    
#        G P #
#        #   #
##############
```

### Global configuration
Global configuration is on a json file. You can here choose player control (speed, acceleration, deceleration) and define tiles availables for your maps.
Each tile can be a dynamic element (door, glass), an ennemy a textured wall or the player start position.
You can see an [example file](/res/conf.json)

## Building and Running the Project

Build the project: Use Cargo, the Rust package manager, to compile the project.

```
cargo build --release
```

Run the project: After building, run the binary:

```
cargo run --release
```
