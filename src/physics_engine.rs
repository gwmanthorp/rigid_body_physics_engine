use crate::rigid_body::RigidBody;
use nalgebra::Vector2;

const WINDOW_WIDTH: f32 = 800.0; // Set to your actual window width
const WINDOW_HEIGHT: f32 = 600.0; // Set to your actual window height

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
                // Apply gravity
                body.velocity += self.gravity * dt;

                // Update position
                body.position += body.velocity * dt;

                // Collision detection with window boundaries
                if body.position.x - body.radius < 0.0 {
                    body.position.x = body.radius;
                    body.velocity.x = -body.velocity.x * DAMPENING_FACTOR;
                } else if body.position.x + body.radius > WINDOW_WIDTH {
                    body.position.x = WINDOW_WIDTH - body.radius;
                    body.velocity.x = -body.velocity.x * DAMPENING_FACTOR;
                }

                if body.position.y - body.radius < 0.0 {
                    body.position.y = body.radius;
                    body.velocity.y = -body.velocity.y * DAMPENING_FACTOR;
                } else if body.position.y + body.radius > WINDOW_HEIGHT {
                    body.position.y = WINDOW_HEIGHT - body.radius;
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
