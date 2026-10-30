# East Engine

**East Engine** is a real-time rendering engine built from scratch using **Rust**.

The engine is being developed with two primary goals:

* 🎮 **Game Development**
* 📊 **Real-Time Visualizations**

The project focuses on understanding how rendering engines work internally — from creating a window and communicating with the GPU to rendering geometry, textures, sprites, text, transforms, cameras, and eventually building a complete engine architecture.

> 🚧 **Status: Early Development — 2D Engine Development**
>
> The Pong prototype has been completed and released as a standalone Windows build. Optimization and cleanup work for the engine foundation has been completed for the current stage. The October development focus is now the **Particle Flow Field visualization demo**.

---

## 🎯 Vision

The long-term goal of East Engine is to become a capable real-time rendering engine that can serve as a foundation for:

* 🎮 Games
* 📊 Interactive visualizations
* 🧪 Simulations
* 🖥️ Graphics applications

Rather than building on top of an existing game engine, East Engine is being developed **from the ground up**.

This allows each system to be implemented and understood individually, from low-level GPU rendering to higher-level engine architecture.

The current priority is to complete the **2D engine foundation** before beginning 3D rendering.

The `master` branch contains the stable engine and the completed October 2026 Particle Flow Field development. Feature work is developed on separate branches and merged after validation.

---

# ✨ Features

## 🎨 Rendering

* Window creation
* GPU initialization
* WGPU rendering
* Rendering pipeline
* Vertex buffers
* Index buffers
* Triangle rendering
* Multiple object rendering
* Object movement
* Texture rendering
* Sprite rendering
* Circle rendering
* Text rendering
* Transform system
* Camera system
* Camera movement
* Camera zoom
* Fullscreen support
* Window resizing
* Separate text and sprite rendering resources
* GPU uniform-based object transforms
* Shared GPU camera uniform buffer
* Cached font loading
* Shared text sampler
* GPU uniform-based text transforms
* Independent text color and sprite color

---

## 📝 Text Rendering

East Engine currently has a dedicated text rendering system supporting:

* TTF font loading
* Font rasterization using `fontdue`
* Dynamic text content
* Font size control
* Position
* Rotation
* Scale
* Color
* Opacity
* Visibility
* Left alignment
* Center alignment
* Right alignment
* Multiline text
* Line spacing
* Letter spacing
* Maximum text width
* Text wrapping
* Text bounds calculation
* Dynamic text bitmap regeneration
* Text revision tracking
* Separate text GPU uniforms
* Screen-space-like text rendering

Text rendering is implemented as a dedicated engine subsystem rather than being embedded directly inside the renderer.

---

## 🎮 Input

* Keyboard input
* WASD → object movement
* I/O → adjustable movement speed
* Q/E → object rotation
* Z/X → object scaling
* Arrow keys → camera movement
* +/- → camera zoom
* F → fullscreen toggle
* Escape → exit application

---

## 🖼️ Assets

Current asset support includes:

* PNG texture loading
* JPG texture loading
* TTF font loading
* Font rasterization
* GPU texture creation
* GPU texture views
* GPU texture samplers

A dedicated asset/resource manager is planned to centralize loading, caching, and lifetime management.

---

## 🌊 Particle Simulation and Visualization

The October milestone adds a real-time particle simulation built around a procedurally generated, time-varying 2D vector field.

* Configurable particle count, up to 1,000,000 particles
* Delta-time based particle movement and lifetime
* Procedural vector-field generation with deterministic seeded patterns
* Smoothly evolving flow-field directions
* Bilinear vector-field sampling
* Particle steering toward sampled field directions
* Automatic particle respawning at boundaries or end of life
* Velocity-dependent color gradients (deep blue through violet to red)
* GPU instanced particle rendering
* Optional particle glow and directional trails
* Runtime-adjustable simulation settings
* FPS and frame-time diagnostics

The simulation uses Rayon to parallelize CPU particle updates. GPU rendering uses instancing so particle data is submitted in batches rather than issuing a separate draw call for every particle.

# 🧩 Engine Architecture

East Engine is organized into independent modules.

Current systems include:

* Application system
* Renderer system
* Game state system
* Input system
* Graphics definitions
* Camera system
* Entity system
* Text rendering system
* Texture system
* Sprite system
* Transform system
* Shader system
* Particle simulation and flow-field systems
* Runtime particle settings
* Performance diagnostics

The architecture is being developed incrementally so that individual engine systems remain understandable and reusable.

---

# 🚀 Current Progress

East Engine is being developed incrementally, with each major system implemented and tested before moving to the next stage.

## Rendering

* [x] Window creation
* [x] GPU initialization
* [x] WGPU rendering
* [x] Rendering pipeline
* [x] Vertex buffers
* [x] Index buffers
* [x] Triangle rendering
* [x] Triangle movement
* [x] Multiple object rendering
* [x] Texture rendering
* [x] Circle rendering
* [x] Text rendering
* [x] Sprite rendering
* [x] Transform system
* [x] Camera system
* [x] Camera movement
* [x] Camera zoom
* [x] Fullscreen toggle
* [x] Window resizing
* [x] Separate sprite and text rendering resources
* [x] GPU-based sprite transforms
* [x] GPU-based text transforms
* [x] Independent per-object uniform buffers
* [x] Independent sprite and text GPU resources
* [x] Shared camera GPU uniform buffer
* [x] Cached font loading
* [x] Shared text sampler
* [x] Glyph metric caching for repeated text layout work

---

## Input

* [x] Keyboard input
* [x] WASD movement
* [x] Movement speed control
* [x] Q/E rotation
* [x] Z/X scaling
* [x] Arrow-key camera movement
* [x] Camera zoom controls
* [x] F fullscreen toggle
* [x] ESC key closes the window

---

## Assets

* [x] PNG texture loading
* [x] JPG texture loading
* [x] TTF font loading
* [x] Font rasterization
* [ ] Asset/resource manager
* [ ] Asset caching
* [ ] Centralized resource lifetime management

---

## Text System

* [x] Dedicated text module
* [x] Font loading
* [x] Text rasterization
* [x] Dynamic text content
* [x] Font size
* [x] Position
* [x] Rotation
* [x] Scale
* [x] Color
* [x] Opacity
* [x] Visibility
* [x] Left alignment
* [x] Center alignment
* [x] Right alignment
* [x] Multiline text
* [x] Line spacing
* [x] Letter spacing
* [x] Maximum width
* [x] Text wrapping
* [x] Text bounds
* [x] Dynamic bitmap regeneration
* [x] Text revision tracking
* [x] Separate text GPU uniforms
* [x] Dynamic score text updates

---

## Entity System

* [x] Entity structure
* [x] Entity IDs
* [x] Entity names
* [x] Entity transforms
* [x] Optional sprite components
* [x] Entity creation
* [x] Entity lookup
* [x] Mutable entity lookup
* [x] Entity movement
* [x] Entity rotation
* [x] Entity scaling

The current entity system is intentionally simple and will be expanded as more engine systems are implemented.

---

# 🌊 October 2026 — Particle Flow Field Demo

The October development milestone delivered a real-time **Particle Flow Field** visualization. It demonstrates East Engine's use as both a 2D rendering engine and a foundation for real-time simulations.

Particles move through a procedurally generated vector field whose directions evolve over time. The demo combines CPU-side simulation, parallel particle updates, GPU instanced rendering, and runtime controls.

## Implemented Features

* [x] Particle data structure and spawning
* [x] Delta-time based movement and lifetime
* [x] Boundary detection and automatic respawning
* [x] Procedural 2D vector-field generation
* [x] Deterministic seeded field generation
* [x] Time-varying, organic flow patterns
* [x] Bilinear vector-field sampling
* [x] Particle steering and configurable flow speed
* [x] Velocity-based particle color gradients
* [x] Runtime particle count adjustment (1,000–1,000,000)
* [x] Runtime flow speed and steering adjustment
* [x] Runtime field scale and grid-resolution adjustment
* [x] Runtime particle-size and field-update-interval adjustment
* [x] Rayon-based parallel CPU particle updates
* [x] GPU instanced particle rendering
* [x] Optional particle glow
* [x] Optional directional particle trails
* [x] FPS and frame-time display with a shared visibility toggle
* [x] In-app settings overlay
* [x] Reset to default simulation settings

## Controls

| Input | Action |
|---|---|
| `F1` | Open or close the particle settings panel |
| `Tab` | Select the next setting |
| `Left` / `Down` | Decrease a selected value or turn a toggle off |
| `Right` / `Up` | Increase a selected value or turn a toggle on |
| `R` | Reset particle settings to defaults |
| `F` | Toggle fullscreen |
| `Esc` | Exit the application |

The settings panel exposes particle count, flow speed, steering, field scale, field resolution, particle size, field update interval, FPS display, glow, and trails. FPS and frame-time visibility are controlled together.

## Performance and Rendering

Particle updates are parallelized across CPU threads using Rayon. The renderer uses GPU instancing to draw large particle populations efficiently. The flow field is regenerated at a configurable interval rather than necessarily rebuilding the entire grid every frame.

A development benchmark reached approximately **40 FPS at 1,000,000 particles and 1080p** in release mode on an AMD system using Radeon 780M integrated graphics. This is a hardware- and configuration-specific development result, not a universal performance target.

## October Outcome

The planned October simulation features are implemented in the current codebase. The milestone established a reusable starting point for future simulation and visualization work. The next development phase shifts back toward general-purpose engine architecture, beginning with centralized asset/resource management.

---

# 🗺️ 2D Engine Roadmap

The goal is to complete the following 2D systems before beginning 3D rendering.

## Core 2D Systems

* [x] Transform system
* [x] Camera system
* [x] Entity/GameObject system
* [x] Sprite rendering
* [x] Text rendering
* [x] Improved text rendering API
* [x] Time / Delta Time system
* [ ] Asset/resource manager
* [ ] Scene system
* [ ] Sprite sheet support
* [ ] Sprite animation
* [ ] Tilemap system
* [ ] Layer/depth ordering
* [ ] Sprite batching
* [ ] Rendering improvements

---

## Optimization Work

Engine and simulation optimizations completed so far include:

* [x] Cache the parsed font so it is loaded only once
* [x] Reuse a shared text sampler instead of creating one for each text resource
* [x] Move camera data into a shared GPU uniform buffer
* [x] Update camera GPU data once per frame
* [x] Reduce unnecessary work during text updates through revision tracking
* [x] Protect game delta time from long resize pauses
* [x] Parallelize particle updates with Rayon
* [x] Render particles through GPU instancing
* [x] Control flow-field regeneration frequency

The October particle benchmark and subsequent optimization work provide a baseline for further CPU- and GPU-side profiling.

---

## Physics and Collision

* [ ] 2D collision detection
* [ ] Collision shapes
* [ ] AABB collision
* [ ] Circle collision
* [ ] Collision response
* [ ] Basic 2D physics
* [ ] Gravity
* [ ] Velocity
* [ ] Acceleration
* [ ] Friction
* [ ] Physics bodies

---

## 2D Lighting

* [ ] Basic 2D lighting
* [ ] Point lights
* [ ] Light attenuation
* [ ] Sprite lighting
* [ ] Normal maps
* [ ] Light blending

---

## UI System

* [ ] UI system
* [ ] UI elements
* [ ] Buttons
* [ ] Panels
* [ ] Labels
* [ ] Input fields
* [ ] UI layout
* [ ] UI anchoring
* [ ] UI interaction

---

## Audio

* [ ] Audio system
* [ ] Sound effects
* [ ] Music playback
* [ ] Audio volume control
* [ ] Spatial audio foundations

---

## Particles

The October demo contains a working particle simulation and rendering path. The following items describe future work toward a generalized, reusable particle system:

* [ ] General-purpose particle emitter API
* [ ] Reusable particle configuration and effect definitions
* [ ] Multiple emitter support
* [ ] Generalized particle lifecycle and behavior modules

---

## Post Processing

* [ ] Render targets
* [ ] Post-processing pipeline
* [ ] Screen effects
* [ ] Bloom
* [ ] Color adjustments
* [ ] Other 2D effects

---

# 🎮 2D Demo Game

A small playable 2D game will be created using East Engine once the core 2D systems are sufficiently complete.

The Pong game was used to validate the engine's core systems outside of isolated rendering tests.

### Current Pong Implementation

The current Pong prototype includes:

* [x] Player 1 and Player 2 entities
* [x] Independent paddle movement
* [x] Player 1 on the left side of the screen
* [x] Player 2 on the right side of the screen
* [x] Vertical paddle movement limits
* [x] Ball movement
* [x] Top and bottom wall collision
* [x] Paddle collision
* [x] Score tracking
* [x] Ball reset after scoring
* [x] Dynamic score text rendering
* [x] Sprite-based paddles and ball
* [x] Pong gameplay loop

### Future Pong Goals

* [ ] Win condition / maximum score
* [ ] Improved collision system
* [ ] Physics-based ball movement
* [ ] Sound effects
* [ ] Start / pause / restart states
* [ ] Main menu
* [ ] Improved UI
* [x] Standalone release
* [x] Test on another computer

The Pong game is an important milestone because it demonstrates that East Engine can be used to build and package an actual playable application rather than only rendering technical test scenes.

---

# 🧱 3D Engine

3D development will begin **only after the major 2D engine requirements are complete**.

Planned 3D systems include:

* [ ] 3D math foundations
* [ ] 3D transforms
* [ ] 3D camera
* [ ] Perspective projection
* [ ] 3D vertex pipeline
* [ ] 3D model loading
* [ ] Mesh system
* [ ] Material system
* [ ] Texture mapping
* [ ] Depth testing
* [ ] Lighting
* [ ] Shadow mapping
* [ ] Normal mapping
* [ ] 3D scene system
* [ ] 3D rendering pipeline

---

# 🏗️ Project Structure

The engine is organized into separate modules so that rendering, input, state, application logic, and engine systems remain independent.

```text
src/
├── main.rs
├── app.rs
├── camera.rs
├── entity.rs
├── renderer.rs
├── graphics.rs
├── state.rs
├── input/
│   └── mod.rs
├── text.rs
├── texture.rs
├── time.rs
├── sprite.rs
├── transform.rs
└── shader.wgsl
```
