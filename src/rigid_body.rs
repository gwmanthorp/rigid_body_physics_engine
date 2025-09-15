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
            if self.mass == 0.0 {
                // treat zero-mass as static/infinite mass: clear forces and do nothing
                self.force = Vector2::new(0.0, 0.0);
                return;
            }
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
        // More robust collision response using inverse mass to avoid division by zero
        let delta = other.position - self.position;
        let distance = delta.norm();
        if distance == 0.0 {
            return; // overlapping exactly; skip to avoid NaNs
        }
        let normal = delta / distance;

        // Relative velocity along the normal
        let relative_velocity = other.velocity - self.velocity;
        let speed = relative_velocity.dot(&normal);

        if speed >= 0.0 {
            return; // moving apart or stationary along normal
        }

        // Positional correction (minimum translation vector) using inverse mass
        let overlap = (self.radius + other.radius) - distance;
        if overlap > 0.0 {
            let inv_mass_self = if self.is_static { 0.0 } else { 1.0 / self.mass };
            let inv_mass_other = if other.is_static {
                0.0
            } else {
                1.0 / other.mass
            };
            let inv_total = inv_mass_self + inv_mass_other;

            if inv_total > 0.0 {
                // small slop/bias to avoid jitter
                let percent = 0.8;
                let correction = normal * (overlap / inv_total) * percent;
                if !self.is_static {
                    self.position -= correction * inv_mass_self;
                }
                if !other.is_static {
                    other.position += correction * inv_mass_other;
                }
            }
        }

        // Elastic collision impulse using inverse mass, avoid 1/mass when mass == 0
        let e = 0.3; // coefficient of restitution
        let inv_mass_self = if self.is_static { 0.0 } else { 1.0 / self.mass };
        let inv_mass_other = if other.is_static {
            0.0
        } else {
            1.0 / other.mass
        };
        let inv_total = inv_mass_self + inv_mass_other;
        if inv_total == 0.0 {
            return; // both static or infinite mass - nothing to do
        }

        let j = -(1.0 + e) * speed / inv_total;

        if !self.is_static {
            self.velocity -= normal * (j * inv_mass_self);
        }
        if !other.is_static {
            other.velocity += normal * (j * inv_mass_other);
        }
    }
}
