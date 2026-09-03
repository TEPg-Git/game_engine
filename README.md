# East Engine

**East Engine** is a real-time rendering engine built from scratch using **Rust**.

The engine is being developed with two primary goals:

* 🎮 **Game Development**
* 📊 **Real-Time Visualizations**

The project focuses on understanding how rendering engines work internally — from creating a window and communicating with the GPU to rendering geometry, textures, sprites, text, transforms, cameras, and eventually building a complete engine architecture.

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
* Sprite rendering
* Circle rendering
* Text rendering
* Transform system
* Camera system
* Camera movement
* Camera zoom
* Fullscreen support
* Window resizing

### Input

* Keyboard input
* WASD → object movement
* I/O → adjustable movement speed
* Q/E → object rotation
* Z/X → object scaling
* Arrow keys → camera movement
* +/- → camera zoom
* Escape → window exit

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
* Separate camera system
* Separate text rendering system
* Separate texture system
* Separate sprite system
* Separate transform system

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
* [x] Sprite rendering
* [x] Transform system
* [x] Camera system
* [x] Camera movement
* [x] Camera zoom
* [x] Fullscreen toggle
* [x] Window resizing

### Input

* [x] Keyboard input
* [x] WASD movement
* [x] Movement speed control
* [x] Q/E rotation
* [x] Z/X scaling
* [x] Arrow-key camera movement
* [x] Camera zoom controls
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
* [x] Camera module
* [x] Text rendering module
* [x] Texture module
* [x] Sprite module
* [x] Transform module
* [x] Modular project structure

### Upcoming

* [ ] Improved text rendering API
* [ ] Entity/GameObject system
* [ ] Asset manager
* [ ] 3D model rendering
* [ ] 3D rendering pipeline
* [ ] Lighting
* [ ] More engine systems

---

## 🏗️ Project Structure

The engine is organized into separate modules so that rendering, input, state, application logic, and engine systems remain independent.

```text
src/
├── main.rs
├── app.rs
├── camera.rs
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
