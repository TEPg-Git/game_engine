# East Engine

**East Engine** is a real-time rendering engine built from scratch using **Rust**.

The engine is being developed with two primary goals:

* 🎮 **Game Development**
* 📊 **Real-Time Visualizations**

The project focuses on understanding how rendering engines work internally — from creating a window and communicating with the GPU to rendering geometry, textures, text, and eventually building a complete engine architecture.

> 🚧 **Status: Early Development**

---

## 🎯 Vision

The long-term goal of East Engine is to become a capable real-time rendering engine that can serve as a foundation for:

* 🎮 Games
* 📊 Interactive visualizations
* 🧪 Simulations
* 🖥️ Graphics applications

Rather than building on top of an existing game engine, East Engine is being developed **from the ground up**.

This allows each system to be implemented and understood individually, from low-level rendering concepts to higher-level engine architecture.

---

## ✨ Features

### Rendering

* Window creation
* GPU initialization
* WGPU rendering
* Rendering pipeline
* Vertex and index buffers
* Triangle rendering
* Multiple object rendering
* Object movement
* Texture rendering
* Circle rendering
* Text rendering
* Fullscreen support
* Window resizing

### Input

* Keyboard input
* WASD movement
* Adjustable movement speed
* Escape key window exit

### Assets

* PNG/JPG texture loading
* TTF font loading
* Font rasterization

### Architecture

* Modular application structure
* Dedicated renderer system
* Dedicated game state
* Separate input system
* Separate graphics definitions
* Separate text rendering system

---

## 🚀 Current Progress

East Engine is being developed incrementally, with each system implemented and tested before moving on to the next.

### Rendering

* [x] Window creation
* [x] GPU initialization
* [x] WGPU rendering
* [x] Rendering pipeline
* [x] Triangle rendering
* [x] Triangle movement
* [x] Multiple object rendering
* [x] Texture rendering
* [x] Circle rendering
* [x] Text rendering
* [x] Fullscreen toggle
* [x] Window resizing

### Input

* [x] Keyboard input
* [x] WASD movement
* [x] Movement speed control
* [x] ESC key closes the window

### Assets

* [x] PNG/JPG texture loading
* [x] TTF font loading
* [x] Font rasterization

### Architecture

* [x] Application system
* [x] Renderer module
* [x] Game state module
* [x] Input module
* [x] Graphics module
* [x] Text rendering module
* [x] Modular project structure

### Upcoming

* [ ] Improved text rendering API
* [ ] Sprite system
* [ ] Camera system
* [ ] Transform system
* [ ] Entity/GameObject system
* [ ] Asset manager
* [ ] 3D model rendering
* [ ] 3D rendering pipeline
* [ ] Lighting
* [ ] More engine systems

---

## 🏗️ Project Structure

The engine is organized into separate modules so that rendering, input, state, and application logic remain independent.

```text
src/
├── main.rs
├── app.rs
├── renderer.rs
├── graphics.rs
├── state.rs
├── text.rs
├── input/
│   └── mod.rs
└── shader.wgsl
```

### Module Responsibilities

| Module        | Responsibility                                                      |
| ------------- | ------------------------------------------------------------------- |
| `main.rs`     | Engine entry point                                                  |
| `app.rs`      | Application lifecycle, window events, and game loop                 |
| `renderer.rs` | GPU resources, rendering pipeline, textures, buffers, and rendering |
| `graphics.rs` | Vertex and uniform data structures                                  |
| `state.rs`    | Game state and movement logic                                       |
| `input/`      | Keyboard input handling                                             |
| `text.rs`     | Font loading and text rasterization                                 |
| `shader.wgsl` | GPU shader code                                                     |

This modular structure allows the engine to grow without keeping the entire implementation inside a single `main.rs` file.

---

## 🧠 What I'm Learning

East Engine is both a rendering engine and a learning project.

While developing the engine, I'm learning about:

* Graphics programming
* GPU architecture
* Rendering pipelines
* GPU buffers
* Vertex and index buffers
* Shaders
* WGSL
* Textures and samplers
* Font rendering
* 2D rendering
* 3D rendering
* Graphics mathematics
* Coordinate systems
* Transformations
* Input handling
* Memory management
* Rust programming
* Real-time rendering
* Game engine architecture
* GPU programming with CUDA
* And much more

---

## 🛠️ Technologies

| Technology   | Purpose                                              |
| ------------ | ---------------------------------------------------- |
| **Rust**     | Core programming language                            |
| **wgpu**     | GPU abstraction and rendering                        |
| **winit**    | Window creation and event handling                   |
| **WGSL**     | GPU shader programming                               |
| **fontdue**  | Font loading and glyph rasterization                 |
| **bytemuck** | Conversion between Rust data and GPU-compatible data |

---

## 🏗️ Development Philosophy

East Engine is built **from the ground up, one system at a time**.

The development process focuses on understanding each layer before moving to the next.

```text
Window
   ↓
GPU
   ↓
Rendering Pipeline
   ↓
Shaders
   ↓
Geometry
   ↓
Textures
   ↓
Text
   ↓
Sprites
   ↓
Transforms
   ↓
Camera
   ↓
Objects
   ↓
Models
   ↓
Lighting
   ↓
Engine Systems
```

The goal is not simply to make things work, but to understand **why they work** and how the individual systems come together to form a real-time rendering engine.

---

## 🚧 Development Status

East Engine is currently in **early development**.

The core 2D rendering foundation is working, including GPU initialization, geometry rendering, textures, circles, text, input, movement, and fullscreen support.

The next stage focuses on building higher-level engine systems on top of this rendering foundation.
