mod physics_engine;
mod rigid_body;

use crate::physics_engine::PhysicsEngine;
use crate::rigid_body::RigidBody;
use macroquad::prelude::*;
use macroquad::ui::{self, hash, root_ui}; // Import the UI module and hash! macro
use nalgebra::Vector2;

struct MainState {
    physics_engine: PhysicsEngine,
    is_mouse_held: bool,
    start_point: Vector2<f32>,
    end_point: Vector2<f32>,
    radius: f32,
    mass: f32,
}

impl MainState {
    pub fn new() -> Self {
        let physics_engine = PhysicsEngine::new(Vector2::new(0.0, 2000.0)); // Gravity downwards

        MainState {
            physics_engine,
            is_mouse_held: false,
            start_point: Vector2::new(0.0, 0.0),
            end_point: Vector2::new(0.0, 0.0),
            radius: 10.0, // Default radius
            mass: 1.0,    // Default mass
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();
        self.physics_engine.update(dt);

        // Handle mouse input
        if self.is_mouse_held {
            if is_mouse_button_down(MouseButton::Left) {
                let mouse_pos = mouse_position();
                self.end_point = Vector2::new(mouse_pos.0, mouse_pos.1);
            } else {
                self.is_mouse_held = false;
                self.physics_engine.delete_starting_pos();

                let starting_velocity = Vector2::new(
                    5.0 * (self.start_point.x - self.end_point.x),
                    5.0 * (self.start_point.y - self.end_point.y),
                );

                self.physics_engine.add_body(RigidBody::new(
                    self.mass,
                    self.start_point,
                    false,
                    Some(starting_velocity),
                    self.radius,
                ));
            }
        } else {
            if is_mouse_button_pressed(MouseButton::Left) {
                let mouse_pos = mouse_position();
                // Check if the mouse is within the slider area
                if mouse_pos.1 > 150.0 {
                    // Adjust this value based on slider height
                    self.is_mouse_held = true;
                    self.start_point = Vector2::new(mouse_pos.0, mouse_pos.1);
                    self.end_point = self.start_point;

                    self.physics_engine.create_starting_pos(RigidBody::new(
                        0.0,
                        self.start_point,
                        true,
                        None,
                        self.radius,
                    ));
                }
            }
        }

        // Update sliders
        root_ui().label(None, "Adjust Radius and Mass:");
        root_ui().slider(hash!(), "Radius", 1.0..100.0, &mut self.radius);
        root_ui().slider(hash!(), "Mass", 0.1..10.0, &mut self.mass);
    }

    pub fn draw(&self) {
        clear_background(BLACK);

        // Draw sliders at the top (display only, no mutable borrow)
        root_ui().label(None, "Adjust Radius and Mass:");
        // Sliders are now only in update(), so just show the values here

        // Draw bodies
        for body in self.physics_engine.bodies() {
            draw_circle(body.position.x, body.position.y, body.radius, WHITE);
        }

        // Draw aiming line when holding mouse
        if self.is_mouse_held {
            draw_line(
                self.start_point.x,
                self.start_point.y,
                self.start_point.x + (self.start_point.x - self.end_point.x),
                self.start_point.y + (self.start_point.y - self.end_point.y),
                5.0,
                WHITE,
            );
        }

        // Show hint text until the first dynamic (non-static) object exists
        let has_dynamic = self.physics_engine.bodies().iter().any(|b| !b.is_static);

        if !has_dynamic {
            let hint = "Drag to generate object";
            let font_size = 30;
            let dims = measure_text(hint, None, font_size, 1.0);
            let x = (screen_width() - dims.width) / 2.0;
            let y = screen_height() / 2.0;
            draw_text(hint, x, y, font_size as f32, WHITE);
        }

        // Draw radius and mass text
        let radius_text = format!("Radius: {:.1}", self.radius);
        draw_text(&radius_text, 10.0, 80.0, 20.0, WHITE);

        let mass_text = format!("Mass: {:.1}", self.mass);
        draw_text(&mass_text, 10.0, 120.0, 20.0, WHITE);
    }
}

#[macroquad::main("Physics Engine")]
async fn main() {
    let mut state = MainState::new();

    loop {
        state.update();
        state.draw();
        next_frame().await;
    }
}
