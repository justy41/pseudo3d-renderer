# Pseudo 3D Renderer

A 2D pseudo-3D renderer written in Rust with raylib. It renders a layered scene with different depth scales and loads tile maps from LDtk data.

## Features

- Three rendering layers: background, ground, and foreground
- Per-layer camera scaling for a pseudo-3D depth effect
- LDtk tile-map rendering
- PNG asset loading
- JSON sprite saves
- Built-in debug editor

## Requirements

- Rust and Cargo
- A supported raylib development environment
- [LLVM](https://llvm.org/) compiler chain

## Running

From the project directory:

```powershell
cargo run
```

The application opens a 1280 by 720 resizable window and renders at a 2560 by 1440 internal resolution.

## Controls

| Input | Action |
| --- | --- |
| `W`, `A`, `S`, `D` | Move the camera |
| `` ` `` | Toggle debug mode |
| `F5` | Save the scene layers |
| `Up`, `Down` | Select a rendering layer in debug mode |
| Mouse wheel | Select a texture in debug mode |
| Left mouse button | Place the selected texture |
| Right mouse button | Remove a texture |

## Project Layout

- `src/core/` contains the renderer, editor, layers, sprites, assets, and tile-map code
- `src/entities/` contains game entities
- `src/scenes/` contains scene definitions
- `src/ldtk/` contains the LDtk Rust data types
- `assets/` contains textures, tile maps, and saved layers

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE).
