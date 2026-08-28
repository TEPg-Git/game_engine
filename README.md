# East Engine

**East Engine** is a rendering engine built from scratch using **Rust**, designed for two primary purposes:

- 🎮 **Game Development**
- 📊 **Real-Time Visualizations**

The project is being developed step by step, with a focus on understanding how rendering engines work internally — from window creation and GPU initialization to rendering pipelines, shaders, textures, models, and eventually a complete engine architecture.

> 🚧 **Status: Early Development**

---

## 🎯 Vision

The long-term goal of **East Engine** is to build a capable real-time rendering engine that can serve as the foundation for both **games** and **interactive visualizations**.

The project is built from the ground up rather than relying on an existing game engine, providing an opportunity to explore the systems behind real-time graphics and engine development.

### Main Applications

**🎮 Games**

East Engine will provide the core rendering and engine systems required to build real-time games.

**📊 Visualizations**

The engine will also support interactive real-time visualizations, simulations, and other graphics-intensive applications.

---

## 🛠️ Technologies

- **Rust** — Core programming language
- **wgpu** — Graphics API abstraction and GPU rendering
- **winit** — Window creation and event handling
- **WGSL** — GPU shader programming
- **bytemuck** — Safe conversion between Rust data and GPU-compatible data

---

## 🚀 Current Progress

The engine is being developed incrementally.

### Rendering

- [x] Window creation
- [x] GPU initialization
- [x] Rendering pipeline
- [x] Triangle rendering
- [x] Triangle movement
- [x] Texture rendering
- [x] Fullscreen toggle
- [x] Multiple object rendering
- [x] ESC key closes the window
- [x] Circle Rendering

### In Progress

- [ ] Independent movement of multiple objects
- [ ] Text rendering
- [ ] 3D model rendering

More features will be added as development continues.

---

## 🧠 What I'm Learning

East Engine is also a learning project focused on understanding the foundations of real-time rendering and engine development.

Through the project, I'm learning and implementing:

- Graphics programming
- GPU architecture and rendering
- Rendering pipelines
- Shader programming
- Vertex and index buffers
- Textures and samplers
- 2D and 3D rendering
- Input handling
- Mathematics for graphics
- Memory management
- Game engine architecture
- Rust programming
- Real-time rendering techniques
- And much more

---

## 🏗️ Development Philosophy

East Engine is being built **from the ground up**, one system at a time.

Instead of trying to build everything at once, the engine is developed incrementally:

```text
Window
   ↓
GPU
   ↓
Rendering Pipeline
   ↓
Geometry
   ↓
Textures
   ↓
Objects
   ↓
Models
   ↓
Engine Systems

Each feature is an opportunity to understand what is happening underneath the abstraction.

---

📂 Project Structure

The project structure will continue to evolve as new engine systems are introduced.

```text
East-Engine/
├── src/
│   ├── ...
│   └── ...
├── shaders/
│   └── ...
├── Cargo.toml
└── README.md
