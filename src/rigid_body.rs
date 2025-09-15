use nalgebra::Vector2;

pub struct RigidBody {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub force: Vector2<f32>,
    pub mass: f32,
    pub is_static: bool,
    pub radius: f32,
}

impl RigidBody {
    pub fn new(
        mass: f32,
        position: Vector2<f32>,
        is_static: bool,
        velocity: Option<Vector2<f32>>,
        radius: f32,
    ) -> Self {
        RigidBody {
            position,
            velocity: velocity.unwrap_or(Vector2::new(0.0, 0.0)),
            force: Vector2::new(0.0, 0.0),
            mass,
            is_static,
            radius,
        }
    }

    pub fn apply_force(&mut self, force: Vector2<f32>) {
        self.force += force;
    }

    pub fn update(&mut self, dt: f32) {
        if !self.is_static {
            let acceleration = self.force / self.mass;
            self.velocity += acceleration * dt;
            self.position += self.velocity * dt;
            self.force = Vector2::new(0.0, 0.0); // Reset force after each update
        }
    }

    pub fn check_collision(&self, other: &RigidBody) -> bool {
        // Simple circle collision detection
        let distance = (self.position - other.position).norm();
        distance < self.radius + other.radius
    }

    pub fn resolve_collision(&mut self, other: &mut RigidBody) {
        // More realistic collision response
        let normal = (other.position - self.position).normalize();
        // Correctly calculate relative velocity
        let relative_velocity = other.velocity - self.velocity;
        let speed = relative_velocity.dot(&normal);

        if speed >= 0.0 {
            return; // They are moving apart or touching perfectly still
        }

        // Prevent overlap
        let distance = (self.position - other.position).norm();
        let overlap = (self.radius + other.radius) - distance;
        if overlap > 0.0 {
            let mtv = normal * overlap; // Minimum translation vector
            let total_mass = self.mass + other.mass;
            // Distribute push-out based on mass
            if !self.is_static {
                self.position -= mtv * (other.mass / total_mass);
            }
            if !other.is_static {
                other.position += mtv * (self.mass / total_mass);
            }
        }

        // Elastic collision response
        let e = 0.3; // Coefficient of restitution
        let j = -(1.0 + e) * speed / (1.0 / self.mass + 1.0 / other.mass);

        if !self.is_static {
            self.velocity -= j / self.mass * normal;
        }
        if !other.is_static {
            other.velocity += j / other.mass * normal;
        }
    }
}
