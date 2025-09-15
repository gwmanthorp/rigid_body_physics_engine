use crate::rigid_body::RigidBody;
use macroquad::window::{screen_height, screen_width};
use nalgebra::Vector2;

const DAMPENING_FACTOR: f32 = 0.8;
const FRICTION_FACTOR: f32 = 0.98;

pub struct PhysicsEngine {
    bodies: Vec<RigidBody>,
    gravity: Vector2<f32>,
}

impl PhysicsEngine {
    pub fn new(gravity: Vector2<f32>) -> Self {
        PhysicsEngine {
            bodies: Vec::new(),
            gravity,
        }
    }

    pub fn create_starting_pos(&mut self, body: RigidBody) {
        self.bodies.push(body);
    }

    pub fn delete_starting_pos(&mut self) {
        self.bodies.pop();
    }

    pub fn add_body(&mut self, body: RigidBody) {
        self.bodies.push(body);
    }

    pub fn update(&mut self, dt: f32) {
        self.handle_collisions();
        for body in &mut self.bodies {
            if !body.is_static {
                // Treat `self.gravity` as acceleration (g). Convert to force F = m * g
                // so acceleration becomes g regardless of mass.
                body.apply_force(self.gravity * body.mass);

                // Let RigidBody integrate velocity & position (and clear forces)
                body.update(dt);

                // Collision detection with window boundaries (same response)
                if body.position.x - body.radius < 0.0 {
                    body.position.x = body.radius;
                    body.velocity.x = -body.velocity.x * DAMPENING_FACTOR;
                } else if body.position.x + body.radius > screen_width() {
                    body.position.x = screen_width() - body.radius;
                    body.velocity.x = -body.velocity.x * DAMPENING_FACTOR;
                }

                if body.position.y - body.radius < 0.0 {
                    body.position.y = body.radius;
                    body.velocity.y = -body.velocity.y * DAMPENING_FACTOR;
                } else if body.position.y + body.radius > screen_height() {
                    body.position.y = screen_height() - body.radius;
                    body.velocity.y = -body.velocity.y * DAMPENING_FACTOR;
                    body.velocity.x *= FRICTION_FACTOR;
                }
            }
        }
    }

    fn handle_collisions(&mut self) {
        // Simple collision detection and response
        for i in 0..self.bodies.len() {
            for j in (i + 1)..self.bodies.len() {
                let (left, right) = self.bodies.split_at_mut(j);
                let body_a = &mut left[i];
                let body_b = &mut right[0];

                // Check for collision and resolve
                if body_a.check_collision(body_b) {
                    body_a.resolve_collision(body_b);
                }
            }
        }
    }

    pub fn bodies(&self) -> &Vec<RigidBody> {
        &self.bodies
    }
}
