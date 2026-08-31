# East Engine

**East Engine** is a real-time rendering engine built from scratch using **Rust**.

The engine is being developed with two primary goals:

- 🎮 **Game Development**
- 📊 **Real-Time Visualizations**

The project focuses on understanding how rendering engines work internally — from creating a window and communicating with the GPU to rendering geometry, textures, text, models, and eventually building a complete engine architecture.

> 🚧 **Status: Early Development**

---

## 🎯 Vision

The long-term goal of East Engine is to become a capable real-time rendering engine that can serve as a foundation for:

- 🎮 Games
- 📊 Interactive visualizations
- 🧪 Simulations
- 🖥️ Graphics applications

Rather than building on top of an existing game engine, East Engine is being developed **from the ground up**.

This allows each system to be implemented and understood individually, from the lowest-level rendering concepts to higher-level engine architecture.

---

## ✨ Features

### Rendering

- Window creation
- GPU initialization
- WGPU rendering pipeline
- Vertex and index buffers
- Triangle rendering
- Multiple object rendering
- Object movement
- Texture rendering
- Circle rendering
- Text rendering
- Fullscreen support

### Input

- Keyboard input
- WASD movement
- Adjustable movement speed
- Escape key window exit

### Assets

- PNG/JPG texture loading
- TTF font loading
- Font rasterization

---

## 🚀 Current Progress

East Engine is being developed incrementally, with new systems added as the project progresses.

### Rendering

- [x] Window creation
- [x] GPU initialization
- [x] Rendering pipeline
- [x] Triangle rendering
- [x] Triangle movement
- [x] Multiple object rendering
- [x] Texture rendering
- [x] Circle rendering
- [x] Text rendering
- [x] Fullscreen toggle

### Input

- [x] Keyboard input
- [x] WASD movement
- [x] Movement speed control
- [x] ESC key closes the window

### Assets

- [x] PNG/JPG texture loading
- [x] TTF font loading
- [x] Font rasterization

### Upcoming

- [ ] Improved text rendering API
- [ ] Sprite system
- [ ] Camera system
- [ ] Entity/GameObject system
- [ ] Asset manager
- [ ] 3D model rendering
- [ ] 3D rendering pipeline
- [ ] More engine systems

---

## 🛠️ Technologies

| Technology | Purpose |
|------------|---------|
| **Rust** | Core programming language |
| **wgpu** | GPU abstraction and rendering |
| **winit** | Window creation and event handling |
| **WGSL** | GPU shader programming |
| **fontdue** | Font loading and glyph rasterization |
| **bytemuck** | Conversion between Rust data and GPU-compatible data |

---

## 🧠 What I'm Learning

East Engine is both a rendering engine and a learning project.

While developing the engine, I'm learning about:

- Graphics programming
- GPU architecture
- Rendering pipelines
- GPU buffers
- Vertex and index buffers
- Shaders
- WGSL
- Textures and samplers
- Font rendering
- 2D rendering
- 3D rendering
- Graphics mathematics
- Coordinate systems
- Transformations
- Input handling
- Memory management
- Rust programming
- Real-time rendering
- Game engine architecture
- GPU programming with CUDA
- And much more

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
Camera
   ↓
Objects
   ↓
Models
   ↓
Engine Systems
