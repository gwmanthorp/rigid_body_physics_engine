# PhysicsEngine — 2D Rigid Body Physics Engine (Rust)

[A 2D Physics Engine](https://gwmanthorp.github.io/rigid_body_physics_engine/)

This project is a simple 2D rigid body physics engine built from scratch using Rust. It uses the macroquad game engine for rendering and the nalgebra library for vector mathematics. Watch circles collide, bounce, and respond to gravity in a fun, interactive simulation!

# 🎯 Overview

The goal of this project was to build a basic physics simulation from the ground up to explore core concepts like collision detection, elastic collision response, and rigid body dynamics. By avoiding pre-built physics libraries, this project provides a hands-on look at the math and logic that power physics in games and simulations.

# ✨ Features

- From-Scratch Physics Core: All physics calculations, including gravity, velocity updates, and collision responses, are implemented manually.
- Circle-Based Rigid Bodies: The engine simulates circular objects with physical properties like mass, position, and velocity.
- Interactive Object Spawning: Click and drag your mouse to launch new balls into the simulation with variable force and direction! 🎯
- Collision Detection & Response: Implements circle-to-circle collision detection and a realistic elastic collision response using a coefficient of restitution to handle bounces.
- Boundary Constraints: Objects realistically bounce off the window's edges with configurable damping and friction.
- Built with Rust: Leverages the performance and safety of the Rust programming language. 🦀
- Macroquad for Rendering: Utilizes the macroquad 2D game framework for drawing objects, handling the game loop, and processing user input.

# 🚀 How to Run

## Prerequisites

- Rust toolchain (install via https://rustup.rs)

## Clone the repository:

```bash
git clone https://github.com/your-username/your-repo-name.git
```

## Navigate to the directory:

```bash
cd your-repo-name
```

## Run the project:

```bash
cargo run --release
```

(The `--release` flag is recommended for smooth performance.)

## Interact:

Once the window opens, click and drag with your left mouse button to launch new objects into the scene!

# 📁 Project Structure

```
rusty-physics-fun/
├── src/
│   ├── main.rs             # Main application loop, rendering, and user input
│   ├── physics_engine.rs   # Manages all physics objects and the simulation loop
│   └── rigid_body.rs       # Defines the RigidBody struct and its logic
├── Cargo.toml              # Project dependencies (macroquad, nalgebra)
└── README.md               # This file
```
