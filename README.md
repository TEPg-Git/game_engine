# East Engine

**East Engine** is a real-time rendering engine built from scratch using **Rust**.

The engine is being developed with two primary goals:

* 🎮 **Game Development**
* 📊 **Real-Time Visualizations**

The project focuses on understanding how rendering engines work internally — from creating a window and communicating with the GPU to rendering geometry, textures, sprites, text, transforms, cameras, and eventually building a complete engine architecture.

> 🚧 **Status: Early Development — 2D Engine Development**

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

# 🗺️ 2D Engine Roadmap

The goal is to complete the following 2D systems before beginning 3D rendering.

## Core 2D Systems

* [x] Transform system
* [x] Camera system
* [x] Entity/GameObject system
* [x] Sprite rendering
* [x] Text rendering
* [x] Improved text rendering API
* [ ] Asset/resource manager
* [ ] Scene system
* [ ] Time / Delta Time system
* [ ] Sprite sheet support
* [ ] Sprite animation
* [ ] Tilemap system
* [ ] Layer/depth ordering
* [ ] Sprite batching
* [ ] Rendering improvements

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

* [ ] Particle system
* [ ] Particle emitters
* [ ] Particle lifetime
* [ ] Particle velocity
* [ ] Particle scaling
* [ ] Particle color
* [ ] Particle effects

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

The demo will be used to validate the engine outside of isolated rendering tests.

Planned goals:

* [ ] Player controller
* [ ] Multiple entities
* [ ] Sprite animation
* [ ] Collision
* [ ] Physics
* [ ] Camera following
* [ ] Tilemap
* [ ] UI
* [ ] Audio
* [ ] Particles
* [ ] Scene loading
* [ ] Basic gameplay loop
* [ ] Build a standalone release
* [ ] Test on another computer

The demo is an important milestone because it will demonstrate that East Engine can be used to build an actual playable application rather than only rendering technical test scenes.

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
├── sprite.rs
├── transform.rs
└── shader.wgsl
